use crate::action::UnitTrace;
use crate::api::{BodyFilter, TextAction};
use crate::filter::error::Result;
use crate::filter::gzip_filter_body::{GzDecodeFilterBody, GzEncodeFilterBody};
use crate::filter::html_body_action::HtmlBodyVisitor;
use crate::filter::text_filter_body::{TextFilterAction, TextFilterBodyAction};
use crate::filter::HtmlFilterBodyAction;
use crate::http::Header;

#[derive(Debug)]
pub struct FilterBodyAction {
    chain: Vec<FilterBodyActionItem>,
    in_error: bool,
}

#[derive(Debug)]
pub enum FilterBodyActionItem {
    Html(HtmlFilterBodyAction),
    Text(TextFilterBodyAction),
    UnGzip(GzDecodeFilterBody),
    Gzip(GzEncodeFilterBody),
}

impl FilterBodyAction {
    pub fn new(filters: Vec<BodyFilter>, headers: &[Header]) -> Self {
        let mut chain = Vec::new();
        let mut has_gzip = false;

        for header in headers {
            if header.name.to_lowercase() == "content-encoding" && header.value.to_lowercase() == "gzip" {
                has_gzip = true;
                break;
            }
        }

        if has_gzip {
            chain.push(FilterBodyActionItem::UnGzip(GzDecodeFilterBody::new()));
        }

        for filter in filters {
            if let Some(item) = FilterBodyActionItem::new(filter) {
                chain.push(item);
            }
        }

        if has_gzip {
            chain.push(FilterBodyActionItem::Gzip(GzEncodeFilterBody::new()));
        }

        Self { chain, in_error: false }
    }

    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }

    pub fn filter(&mut self, data: Vec<u8>, mut unit_trace: Option<&mut UnitTrace>) -> Vec<u8> {
        if self.in_error {
            return data;
        }

        match self.do_filter(data.clone(), unit_trace.as_deref_mut()) {
            Ok(filtered) => filtered,
            Err(err) => {
                log::error!("error while filtering: {}", err);
                self.in_error = true;

                data
            }
        }
    }

    fn do_filter(&mut self, mut data: Vec<u8>, mut unit_trace: Option<&mut UnitTrace>) -> Result<Vec<u8>> {
        for item in &mut self.chain {
            data = item.filter(data, unit_trace.as_deref_mut())?;

            if data.is_empty() {
                break;
            }
        }

        Ok(data)
    }

    pub fn end(&mut self, mut unit_trace: Option<&mut UnitTrace>) -> Vec<u8> {
        if self.in_error {
            return Vec::new();
        }

        match self.do_end(unit_trace.as_deref_mut()) {
            Ok(end) => end,
            Err(err) => {
                log::error!("error while ending filtering: {}", err);
                self.in_error = true;

                Vec::new()
            }
        }
    }

    fn do_end(&mut self, mut unit_trace: Option<&mut UnitTrace>) -> Result<Vec<u8>> {
        let mut data = None;

        for item in &mut self.chain {
            let new_data = match data {
                None => item.end()?,
                Some(str) => {
                    let mut end_str = item.filter(str, unit_trace.as_deref_mut())?;
                    end_str.extend(item.end()?);

                    end_str
                }
            };

            data = if new_data.is_empty() { None } else { Some(new_data) };
        }

        Ok(data.unwrap_or_default())
    }
}

impl FilterBodyActionItem {
    pub fn new(filter: BodyFilter) -> Option<Self> {
        match filter {
            BodyFilter::HTML(html_body_filter) => {
                HtmlBodyVisitor::new(html_body_filter).map(|visitor| Self::Html(HtmlFilterBodyAction::new(visitor)))
            }
            BodyFilter::Text(text_body_filter) => Some(Self::Text(TextFilterBodyAction::new(
                text_body_filter.id,
                match text_body_filter.action {
                    TextAction::Append => TextFilterAction::Append,
                    TextAction::Prepend => TextFilterAction::Prepend,
                    TextAction::Replace => TextFilterAction::Replace,
                },
                text_body_filter.content,
            ))),
        }
    }

    pub fn filter(&mut self, data: Vec<u8>, mut unit_trace: Option<&mut UnitTrace>) -> Result<Vec<u8>> {
        Ok(match self {
            FilterBodyActionItem::Html(html_body_filter) => html_body_filter.filter(data, unit_trace.as_deref_mut())?,
            FilterBodyActionItem::Text(text_body_filter) => text_body_filter.filter(data, unit_trace.as_deref_mut()),
            FilterBodyActionItem::UnGzip(gzip_body_filter) => gzip_body_filter.filter(data)?,
            FilterBodyActionItem::Gzip(gzip_body_filter) => gzip_body_filter.filter(data)?,
        })
    }

    pub fn end(&mut self) -> Result<Vec<u8>> {
        Ok(match self {
            FilterBodyActionItem::Html(html_body_filter) => html_body_filter.end(),
            FilterBodyActionItem::Text(text_body_filter) => text_body_filter.end(),
            FilterBodyActionItem::UnGzip(gzip_body_filter) => gzip_body_filter.end()?,
            FilterBodyActionItem::Gzip(gzip_body_filter) => gzip_body_filter.end()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::HTMLBodyFilter;
    use flate2::write::{GzDecoder, GzEncoder};
    use flate2::Compression;
    use std::io::prelude::*;

    #[test]
    pub fn test_filter_gzip() {
        let before_filter = "Test".to_string();
        let mut e = GzEncoder::new(Vec::new(), Compression::default());
        e.write_all(before_filter.as_bytes()).unwrap();
        let bytes = e.finish().unwrap();

        let headers = vec![Header {
            name: "Content-Encoding".to_string(),
            value: "gzip".to_string(),
        }];
        let mut filter = FilterBodyAction::new(Vec::new(), &headers);

        let mut filtered = filter.filter(bytes.clone(), None);
        filtered.extend(filter.end(None));

        let mut gz = GzDecoder::new(Vec::new());
        gz.write_all(&filtered).unwrap();
        let data = gz.finish().unwrap();
        let after_filter = String::from_utf8(data.to_vec()).unwrap();

        assert_eq!(before_filter, after_filter);
    }

    #[test]
    pub fn test_filter() {
        let mut filter = FilterBodyAction::new(Vec::new(), &[]);

        let before_filter = "Test".to_string().into_bytes();
        let filtered = filter.filter(before_filter.clone(), None);
        let end = filter.end(None);

        assert_eq!(before_filter, filtered);
        assert_eq!(true, end.is_empty());
    }

    #[test]
    pub fn test_buffer_on_error() {
        let mut filter = FilterBodyAction::new(Vec::new(), &[]);

        let mut filtered = filter.filter("<div>Text </".to_string().into_bytes(), None);
        filtered.extend(filter.end(None));

        assert_eq!("<div>Text </".to_string().into_bytes(), filtered);
    }

    #[test]
    pub fn test_replace() {
        let mut filter = FilterBodyAction::new(
            vec![
                BodyFilter::HTML(HTMLBodyFilter {
                    action: "append_child".to_string(),
                    element_tree: vec!["html".to_string(), "head".to_string()],
                    css_selector: Some(r#"meta[name="description"]"#.to_string()),
                    value: "<meta name=\"description\" content=\"New Description\" />".to_string(),
                    id: Some("test".to_string()),
                    target_hash: Some("target_hash".to_string()),
                    inner_value: None,
                }),
                BodyFilter::HTML(HTMLBodyFilter {
                    action: "replace".to_string(),
                    element_tree: vec!["html".to_string(), "head".to_string(), "meta".to_string()],
                    css_selector: Some(r#"meta[name="description"]"#.to_string()),
                    value: "<meta name=\"description\" content=\"New Description\" />".to_string(),
                    id: Some("test".to_string()),
                    target_hash: Some("target_hash".to_string()),
                    inner_value: None,
                }),
            ],
            &[],
        );

        let mut filtered = filter.filter(
            "<html><head><meta name=\"description\"></head></html>".to_string().into_bytes(),
            None,
        );
        filtered.extend(filter.end(None));

        assert_eq!(
            "<html><head><meta name=\"description\" content=\"New Description\" /></head></html>"
                .to_string()
                .into_bytes(),
            filtered
        );
    }

    #[test]
    pub fn test_append() {
        let mut filter = FilterBodyAction::new(
            vec![
                BodyFilter::HTML(HTMLBodyFilter {
                    action: "append_child".to_string(),
                    element_tree: vec!["html".to_string(), "head".to_string()],
                    css_selector: Some(r#"meta[name="description"]"#.to_string()),
                    value: "<meta name=\"description\" content=\"New Description\" />".to_string(),
                    id: Some("test".to_string()),
                    target_hash: Some("target_hash".to_string()),
                    inner_value: None,
                }),
                BodyFilter::HTML(HTMLBodyFilter {
                    action: "replace".to_string(),
                    element_tree: vec!["html".to_string(), "head".to_string(), "meta".to_string()],
                    css_selector: Some(r#"meta[name="description"]"#.to_string()),
                    value: "<meta name=\"description\" content=\"New Description\" />".to_string(),
                    id: Some("test".to_string()),
                    target_hash: Some("target_hash".to_string()),
                    inner_value: None,
                }),
            ],
            &[],
        );

        let mut filtered = filter.filter("<html><head><meta></head></html>".to_string().into_bytes(), None);
        filtered.extend(filter.end(None));

        assert_eq!(
            "<html><head><meta><meta name=\"description\" content=\"New Description\" /></head></html>"
                .to_string()
                .into_bytes(),
            filtered
        );
    }

    #[test]
    pub fn test_prepend() {
        let mut filter = FilterBodyAction::new(
            vec![BodyFilter::HTML(HTMLBodyFilter {
                action: "prepend_child".to_string(),
                element_tree: vec!["html".to_string(), "body".to_string()],
                css_selector: Some("".to_string()),
                value: "<p>This is as test</p>".to_string(),
                id: Some("test".to_string()),
                target_hash: Some("target_hash".to_string()),
                inner_value: None,
            })],
            &[],
        );

        let mut filtered = filter.filter(
            "<html><head></head><body class=\"page\"><div>Yolo</div></body></html>"
                .to_string()
                .into_bytes(),
            None,
        );
        filtered.extend(filter.end(None));

        assert_eq!(
            "<html><head></head><body class=\"page\"><p>This is as test</p><div>Yolo</div></body></html>"
                .to_string()
                .into_bytes(),
            filtered
        );
    }

    #[test]
    pub fn test_description_2() {
        let mut filter = FilterBodyAction::new(
            vec![
                BodyFilter::HTML(HTMLBodyFilter {
                    action: "append_child".to_string(),
                    element_tree: vec!["html".to_string(), "head".to_string()],
                    css_selector: Some(r#"meta[property="og:description"]"#.to_string()),
                    value: r#"<meta property="og:description" content="New Description" />"#.to_string(),
                    id: Some("test".to_string()),
                    target_hash: Some("target_hash".to_string()),
                    inner_value: None,
                }),
                BodyFilter::HTML(HTMLBodyFilter {
                    action: "replace".to_string(),
                    element_tree: vec!["html".to_string(), "head".to_string(), "meta".to_string()],
                    css_selector: Some(r#"meta[property="og:description"]"#.to_string()),
                    value: r#"<meta property="og:description" content="New Description" />"#.to_string(),
                    id: Some("test".to_string()),
                    target_hash: Some("target_hash".to_string()),
                    inner_value: None,
                }),
            ],
            &[],
        );

        let mut filtered = filter.filter(r#"<html><head><description>Old description</description><meta /><meta property="og:description" content="Old Description" /></head></html>"#.to_string().into_bytes(), None);
        filtered.extend(filter.end(None));

        assert_eq!(
            r#"<html><head><description>Old description</description><meta /><meta property="og:description" content="New Description" /></head></html>"#.to_string().into_bytes(),
            filtered
        );
    }
}
