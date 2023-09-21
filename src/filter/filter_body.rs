use crate::action::UnitTrace;
use crate::api::{BodyFilter, TextAction};
#[cfg(feature = "compress")]
use crate::filter::encoding::{get_encoding_filters, DecodeFilterBody, EncodeFilterBody};
use crate::filter::error::Result;
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
    #[cfg(feature = "compress")]
    Encode(Box<EncodeFilterBody>),
    #[cfg(feature = "compress")]
    Decode(Box<DecodeFilterBody>),
}

impl FilterBodyAction {
    pub fn new(filters: Vec<BodyFilter>, headers: &[Header]) -> Self {
        let mut chain = Vec::new();
        let mut content_type = None;
        #[cfg(feature = "compress")]
        let mut content_encoding = None;

        for header in headers {
            #[cfg(feature = "compress")]
            if header.name.to_lowercase() == "content-encoding" {
                content_encoding = Some(header.value.to_lowercase());
            }

            if header.name.to_lowercase() == "content-type" {
                content_type = Some(header.value.to_lowercase());
            }
        }

        for filter in filters {
            if let Some(item) = FilterBodyActionItem::new(filter, content_type.clone()) {
                chain.push(item);
            }
        }

        #[cfg(not(feature = "compress"))]
        {
            return Self { chain, in_error: false };
        }

        #[cfg(feature = "compress")]
        if chain.is_empty() {
            return Self { chain, in_error: false };
        }

        #[cfg(feature = "compress")]
        match content_encoding {
            Some(encoding) => match get_encoding_filters(encoding.as_str()) {
                Some((decode, encode)) => {
                    chain.insert(0, FilterBodyActionItem::Decode(Box::new(decode)));
                    chain.push(FilterBodyActionItem::Encode(Box::new(encode)));

                    Self { chain, in_error: false }
                }
                None => {
                    log::error!(
                        "redirectionio does not support content-encoding {}, filtering will be disable for this request",
                        encoding
                    );

                    Self {
                        chain: Vec::new(),
                        in_error: false,
                    }
                }
            },
            None => Self { chain, in_error: false },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }

    pub fn filter(&mut self, data: Vec<u8>, unit_trace: Option<&mut UnitTrace>) -> Vec<u8> {
        if self.in_error {
            return data;
        }

        match self.do_filter(data.clone(), unit_trace) {
            Ok(filtered) => filtered,
            Err(err) => {
                log::error!("error while filtering: {:?}", err);
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

    pub fn end(&mut self, unit_trace: Option<&mut UnitTrace>) -> Vec<u8> {
        if self.in_error {
            return Vec::new();
        }

        match self.do_end(unit_trace) {
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
    pub fn new(filter: BodyFilter, content_type: Option<String>) -> Option<Self> {
        match filter {
            BodyFilter::HTML(html_body_filter) => match content_type {
                Some(content_type) if content_type.contains("text/html") => {
                    // @TODO Support charset
                    HtmlBodyVisitor::new(html_body_filter).map(|visitor| Self::Html(HtmlFilterBodyAction::new(visitor)))
                }
                None => {
                    // Assume HTML if no content type
                    HtmlBodyVisitor::new(html_body_filter).map(|visitor| Self::Html(HtmlFilterBodyAction::new(visitor)))
                }
                _ => {
                    log::error!(
                        "html filtering is only supported for text/html content type, {} received",
                        content_type.unwrap_or_default()
                    );

                    None
                }
            },
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

    pub fn filter(&mut self, data: Vec<u8>, unit_trace: Option<&mut UnitTrace>) -> Result<Vec<u8>> {
        Ok(match self {
            FilterBodyActionItem::Html(html_body_filter) => html_body_filter.filter(data, unit_trace)?,
            FilterBodyActionItem::Text(text_body_filter) => text_body_filter.filter(data, unit_trace),
            #[cfg(feature = "compress")]
            FilterBodyActionItem::Decode(decode_body_filter) => decode_body_filter.filter(data)?,
            #[cfg(feature = "compress")]
            FilterBodyActionItem::Encode(encode_body_filter) => encode_body_filter.filter(data)?,
        })
    }

    pub fn end(&mut self) -> Result<Vec<u8>> {
        Ok(match self {
            FilterBodyActionItem::Html(html_body_filter) => html_body_filter.end(),
            FilterBodyActionItem::Text(text_body_filter) => text_body_filter.end(),
            #[cfg(feature = "compress")]
            FilterBodyActionItem::Decode(decode_body_filter) => decode_body_filter.end()?,
            #[cfg(feature = "compress")]
            FilterBodyActionItem::Encode(encode_body_filter) => encode_body_filter.end()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::HTMLBodyFilter;
    use flate2::write::{GzDecoder, GzEncoder, ZlibDecoder, ZlibEncoder};
    use flate2::Compression;
    use std::io::prelude::*;

    #[test]
    pub fn test_filter_gzip() {
        let decompressed_input = "<html><head></head><body class=\"page\"><div>Yolo</div></body></html>".to_string();
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(decompressed_input.as_bytes()).unwrap();
        let compressed_input = encoder.finish().unwrap();

        let headers = vec![
            Header {
                name: "Content-Encoding".to_string(),
                value: "gzip".to_string(),
            },
            Header {
                name: "Content-Type".to_string(),
                value: "text/html;charset=".to_string(),
            },
        ];

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
            &headers,
        );

        let size = compressed_input.len();
        let mut filtered_size = 0;
        let mut filtered = Vec::new();

        while filtered_size < size - 10 {
            filtered.extend(filter.filter(compressed_input.as_slice()[filtered_size..filtered_size + 10].to_vec(), None));
            filtered_size += 10;
        }

        filtered.extend(filter.filter(compressed_input.as_slice()[filtered_size..size].to_vec(), None));
        filtered.extend(filter.end(None));

        let mut decoder = GzDecoder::new(Vec::new());
        decoder.write_all(&filtered).unwrap();
        let decompressed_output = decoder.finish().unwrap();

        assert_eq!(
            String::from_utf8(decompressed_output).unwrap(),
            "<html><head></head><body class=\"page\"><p>This is as test</p><div>Yolo</div></body></html>".to_string()
        );
    }

    #[test]
    pub fn test_filter_deflate() {
        let decompressed_input = "<html><head></head><body class=\"page\"><div>Yolo</div></body></html>".to_string();
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(decompressed_input.as_bytes()).unwrap();
        let compressed_input = encoder.finish().unwrap();

        let headers = vec![
            Header {
                name: "Content-Encoding".to_string(),
                value: "deflate".to_string(),
            },
            Header {
                name: "Content-Type".to_string(),
                value: "text/html;charset=".to_string(),
            },
        ];

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
            &headers,
        );

        let size = compressed_input.len();
        let mut filtered_size = 0;
        let mut filtered = Vec::new();

        while filtered_size < size - 10 {
            filtered.extend(filter.filter(compressed_input.as_slice()[filtered_size..filtered_size + 10].to_vec(), None));
            filtered_size += 10;
        }

        filtered.extend(filter.filter(compressed_input.as_slice()[filtered_size..size].to_vec(), None));
        filtered.extend(filter.end(None));

        let mut decoder = ZlibDecoder::new(Vec::new());
        decoder.write_all(&filtered).unwrap();
        let decompressed_output = decoder.finish().unwrap();

        assert_eq!(
            String::from_utf8(decompressed_output).unwrap(),
            "<html><head></head><body class=\"page\"><p>This is as test</p><div>Yolo</div></body></html>".to_string()
        );
    }

    #[test]
    pub fn test_filter_brotli() {
        let decompressed_input = "<html><head><h2>This is stupide data to ensure compression before</H2></head><body class=\"page\"><div>Yolo</div></body></html>".to_string();
        let mut compressed_input = Vec::new();
        let mut reader = brotli::CompressorReader::new(decompressed_input.as_bytes(), 4096, 11, 22);
        reader.read_to_end(&mut compressed_input).expect("Failed to encode");

        let headers = vec![
            Header {
                name: "Content-Encoding".to_string(),
                value: "br".to_string(),
            },
            Header {
                name: "Content-Type".to_string(),
                value: "text/html;charset=".to_string(),
            },
        ];

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
            &headers,
        );

        let size = compressed_input.len();
        let mut filtered_size = 0;
        let mut filtered = Vec::new();

        while filtered_size < size - 10 {
            filtered.extend(filter.filter(compressed_input.as_slice()[filtered_size..filtered_size + 10].to_vec(), None));
            filtered_size += 10;
        }

        filtered.extend(filter.filter(compressed_input.as_slice()[filtered_size..size].to_vec(), None));
        filtered.extend(filter.end(None));

        let mut decompressed_output = Vec::new();
        let mut reader = brotli::Decompressor::new(filtered.as_slice(), 4096);
        reader.read_to_end(&mut decompressed_output).expect("Failed to decompress");

        assert_eq!(
            String::from_utf8(decompressed_output).unwrap(),
            "<html><head><h2>This is stupide data to ensure compression before</H2></head><body class=\"page\"><p>This is as test</p><div>Yolo</div></body></html>".to_string()
        );
    }

    #[test]
    pub fn test_filter() {
        let mut filter = FilterBodyAction::new(Vec::new(), &[]);

        let before_filter = "Test".to_string().into_bytes();
        let filtered = filter.filter(before_filter.clone(), None);
        let end = filter.end(None);

        assert_eq!(before_filter, filtered);
        assert!(end.is_empty());
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
