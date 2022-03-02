use crate::api::{BodyFilter, TextAction};
use crate::filter::html_body_action::HtmlBodyVisitor;
use crate::filter::text_filter_body::{TextFilterAction, TextFilterBodyAction};
use crate::filter::HtmlFilterBodyAction;

pub struct FilterBodyAction {
    chain: Vec<FilterBodyActionItem>,
}

pub enum FilterBodyActionItem {
    Html(HtmlFilterBodyAction),
    Text(TextFilterBodyAction),
}

impl FilterBodyAction {
    pub fn new(filters: Vec<BodyFilter>) -> Self {
        let mut chain = Vec::new();

        for filter in filters {
            if let Some(item) = FilterBodyActionItem::new(filter) {
                chain.push(item);
            }
        }

        Self { chain }
    }

    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
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
            BodyFilter::HTML(html_body_filter) => {
                HtmlBodyVisitor::new(html_body_filter).map(|visitor| Self::Html(HtmlFilterBodyAction::new(visitor)))
            }
            BodyFilter::Text(text_body_filter) => Some(Self::Text(TextFilterBodyAction::new(
                match text_body_filter.action {
                    TextAction::Append => TextFilterAction::Append,
                    TextAction::Prepend => TextFilterAction::Prepend,
                    TextAction::Replace => TextFilterAction::Replace,
                },
                text_body_filter.content,
            ))),
        }
    }

    pub fn filter(&mut self, data: String) -> String {
        match self {
            FilterBodyActionItem::Html(html_body_filter) => html_body_filter.filter(data),
            FilterBodyActionItem::Text(text_body_filter) => text_body_filter.filter(data),
        }
    }

    pub fn end(&mut self) -> String {
        match self {
            FilterBodyActionItem::Html(html_body_filter) => html_body_filter.end(),
            FilterBodyActionItem::Text(text_body_filter) => text_body_filter.end(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::HTMLBodyFilter;

    #[test]
    pub fn test_filter() {
        let mut filter = FilterBodyAction::new(Vec::new());

        let before_filter = "Test".to_string();
        let filtered = filter.filter(before_filter.clone());
        let end = filter.end();

        assert_eq!(before_filter, filtered);
        assert_eq!(true, end.is_empty());
    }

    #[test]
    pub fn test_buffer_on_error() {
        let mut filter = FilterBodyAction::new(Vec::new());

        let mut filtered = filter.filter("<div>Text </".to_string());
        filtered.push_str(filter.end().as_str());

        assert_eq!("<div>Text </", filtered);
    }

    #[test]
    pub fn test_replace() {
        let mut filter = FilterBodyAction::new(vec![
            BodyFilter::HTML(HTMLBodyFilter {
                action: "append_child".to_string(),
                element_tree: vec!["html".to_string(), "head".to_string()],
                css_selector: Some(r#"meta[name="description"]"#.to_string()),
                value: "<meta name=\"description\" content=\"New Description\" />".to_string(),
            }),
            BodyFilter::HTML(HTMLBodyFilter {
                action: "replace".to_string(),
                element_tree: vec!["html".to_string(), "head".to_string(), "meta".to_string()],
                css_selector: Some(r#"meta[name="description"]"#.to_string()),
                value: "<meta name=\"description\" content=\"New Description\" />".to_string(),
            }),
        ]);

        let mut filtered = filter.filter("<html><head><meta name=\"description\"></head></html>".to_string());
        filtered.push_str(filter.end().as_str());

        assert_eq!(
            "<html><head><meta name=\"description\" content=\"New Description\" /></head></html>",
            filtered
        );
    }

    #[test]
    pub fn test_append() {
        let mut filter = FilterBodyAction::new(vec![
            BodyFilter::HTML(HTMLBodyFilter {
                action: "append_child".to_string(),
                element_tree: vec!["html".to_string(), "head".to_string()],
                css_selector: Some(r#"meta[name="description"]"#.to_string()),
                value: "<meta name=\"description\" content=\"New Description\" />".to_string(),
            }),
            BodyFilter::HTML(HTMLBodyFilter {
                action: "replace".to_string(),
                element_tree: vec!["html".to_string(), "head".to_string(), "meta".to_string()],
                css_selector: Some(r#"meta[name="description"]"#.to_string()),
                value: "<meta name=\"description\" content=\"New Description\" />".to_string(),
            }),
        ]);

        let mut filtered = filter.filter("<html><head><meta></head></html>".to_string());
        let end = filter.end();

        filtered.push_str(end.as_str());

        assert_eq!(
            "<html><head><meta><meta name=\"description\" content=\"New Description\" /></head></html>",
            filtered
        );
    }

    #[test]
    pub fn test_prepend() {
        let mut filter = FilterBodyAction::new(vec![BodyFilter::HTML(HTMLBodyFilter {
            action: "prepend_child".to_string(),
            element_tree: vec!["html".to_string(), "body".to_string()],
            css_selector: Some("".to_string()),
            value: "<p>This is as test</p>".to_string(),
        })]);

        let mut filtered = filter.filter("<html><head></head><body class=\"page\"><div>Yolo</div></body></html>".to_string());
        let end = filter.end();

        filtered.push_str(end.as_str());

        assert_eq!(
            "<html><head></head><body class=\"page\"><p>This is as test</p><div>Yolo</div></body></html>",
            filtered
        );
    }

    #[test]
    pub fn test_description_2() {
        let mut filter = FilterBodyAction::new(vec![
            BodyFilter::HTML(HTMLBodyFilter {
                action: "append_child".to_string(),
                element_tree: vec!["html".to_string(), "head".to_string()],
                css_selector: Some(r#"meta[property="og:description"]"#.to_string()),
                value: r#"<meta property="og:description" content="New Description" />"#.to_string(),
            }),
            BodyFilter::HTML(HTMLBodyFilter {
                action: "replace".to_string(),
                element_tree: vec!["html".to_string(), "head".to_string(), "meta".to_string()],
                css_selector: Some(r#"meta[property="og:description"]"#.to_string()),
                value: r#"<meta property="og:description" content="New Description" />"#.to_string(),
            }),
        ]);

        let mut filtered = filter.filter(r#"<html><head><description>Old description</description><meta /><meta property="og:description" content="Old Description" /></head></html>"#.to_string());
        let end = filter.end();

        filtered.push_str(end.as_str());

        assert_eq!(
            r#"<html><head><description>Old description</description><meta /><meta property="og:description" content="New Description" /></head></html>"#,
            filtered
        );
    }
}
