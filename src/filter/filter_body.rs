use crate::api::BodyFilter;
use crate::filter::html_body_action::HtmlBodyVisitor;
use crate::filter::HtmlFilterBodyAction;

pub struct FilterBodyAction {
    chain: Vec<FilterBodyActionItem>,
}

pub enum FilterBodyActionItem {
    HTML(HtmlFilterBodyAction),
}

impl FilterBodyAction {
    pub fn new(filters: Vec<BodyFilter>) -> Option<Self> {
        let mut chain = Vec::new();

        for filter in filters {
            if let Some(item) = FilterBodyActionItem::new(filter) {
                chain.push(item);
            }
        }

        if chain.is_empty() {
            None
        } else {
            Some(Self { chain })
        }
    }

    pub fn filter(&mut self, mut data: String) -> String {
        for item in &mut self.chain {
            data = item.filter(data);

            if data.is_empty() {
                break;
            }
        }

        data
    }

    pub fn end(&mut self) -> String {
        let mut data = None;

        for item in &mut self.chain {
            let new_data = match data {
                None => item.end(),
                Some(str) => {
                    let mut end_str = item.filter(str);
                    end_str.push_str(item.end().as_str());

                    end_str
                }
            };

            data = if new_data.is_empty() { None } else { Some(new_data) };
        }

        data.unwrap_or_else(|| "".to_string())
    }
}

impl FilterBodyActionItem {
    pub fn new(filter: BodyFilter) -> Option<Self> {
        match filter {
            BodyFilter::HTML(html_body_filter) => match HtmlBodyVisitor::new(html_body_filter) {
                None => None,
                Some(visitor) => Some(Self::HTML(HtmlFilterBodyAction::new(visitor))),
            },
        }
    }

    pub fn filter(&mut self, data: String) -> String {
        match self {
            FilterBodyActionItem::HTML(html_body_filter) => html_body_filter.filter(data),
        }
    }

    pub fn end(&mut self) -> String {
        "".to_string()
    }
}
