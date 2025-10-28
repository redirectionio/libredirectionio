use std::{
    borrow::Cow,
    collections::HashMap,
    ops::Deref,
    sync::{Arc, RwLock},
};

use lol_html::{ElementContentHandlers, Settings, html_content::TextChunk};

use crate::{api::VariableValue, marker::StaticOrDynamic};

#[derive(Debug)]
pub struct CaptureRegistry {
    pub variables: HashMap<String, Arc<RwLock<VariableValue>>>,
}

impl CaptureRegistry {
    pub fn replace(&self, value: String) -> String {
        let mut variables = self
            .variables
            .iter()
            .map(|(k, v)| {
                let read_lock = v.read();
                let Ok(val) = read_lock else {
                    return (k.clone(), None);
                };

                (k.clone(), Some(val.deref().clone()))
            })
            .filter_map(|(k, v)| v.map(|val| (k, val)))
            .collect::<Vec<(String, VariableValue)>>();

        // sort by length descending to replace longer keys first
        variables.sort_by(|(key_a, _), (key_b, _)| key_b.len().cmp(&key_a.len()));

        StaticOrDynamic::replace(value, &variables, true)
    }

    pub fn set_variable(&self, name: String, value: String) {
        if let Some(var) = self.variables.get(&name)
            && let Ok(mut write_lock) = var.write()
        {
            *write_lock = write_lock.deref().to_static(value)
        }
    }

    pub fn from_variables(variables: Vec<(String, VariableValue)>) -> Self {
        let mut vars_map = HashMap::new();
        for (name, value) in variables {
            vars_map.insert(name, Arc::new(RwLock::new(value)));
        }
        CaptureRegistry { variables: vars_map }
    }

    pub fn need_body_capture(&self) -> bool {
        for (_, var) in self.variables.iter() {
            if let Ok(read_lock) = var.read()
                && let VariableValue::HtmlFilter { .. } = read_lock.deref()
            {
                return true;
            }
        }

        false
    }

    pub fn selectors(&self) -> Vec<(String, String)> {
        let mut selectors = Vec::new();

        for (name, var) in self.variables.iter() {
            if let Ok(read_lock) = var.read()
                && let VariableValue::HtmlFilter { selector, .. } = read_lock.deref()
            {
                selectors.push((selector.clone(), name.clone()));
            }
        }

        selectors
    }
}

#[derive(Debug)]
pub struct BodyCapture(pub Arc<CaptureRegistry>);

impl BodyCapture {
    pub fn into_handlers(self, settings: &mut Settings) {
        for (selector, variable_name) in self.0.selectors() {
            let css_selector = match selector.parse() {
                Ok(selector) => selector,
                Err(_) => {
                    log::error!("Failed to parse CSS selector: {}", selector);
                    continue;
                }
            };

            let mut value = String::new();
            let variables = self.0.clone();
            settings.element_content_handlers.push((
                Cow::Owned(css_selector),
                ElementContentHandlers::default().text(move |text: &mut TextChunk| {
                    value += text.as_str();

                    if text.last_in_text_node() {
                        variables.set_variable(variable_name.clone(), value.clone());
                        value = String::new();
                    }

                    Ok(())
                }),
            ));
        }
    }
}
