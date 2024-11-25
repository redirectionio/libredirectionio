use http::uri::PathAndQuery;
use linked_hash_map::LinkedHashMap;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use serde::{Deserialize, Serialize};
use url::form_urlencoded::parse as parse_query;

use crate::router_config::RouterConfig;

const URL_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>').add(b'+');

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct PathAndQueryWithSkipped {
    pub path_and_query: String,
    pub path_and_query_matching: Option<String>,
    pub skipped_query_params: Option<String>,
    pub original: String,
}

pub fn sanitize_url(path_and_query_str: &str) -> String {
    utf8_percent_encode(path_and_query_str, URL_ENCODE_SET).to_string()
}

impl PathAndQueryWithSkipped {
    pub fn from_static(path_and_query_str: &str) -> Self {
        let url = sanitize_url(path_and_query_str);

        Self {
            path_and_query_matching: Some(path_and_query_str.to_string()),
            path_and_query: url,
            original: path_and_query_str.to_string(),
            skipped_query_params: None,
        }
    }

    pub fn from_config(config: &RouterConfig, path_and_query_str: &str) -> Self {
        let url = sanitize_url(path_and_query_str);

        if !config.ignore_marketing_query_params && !config.ignore_all_query_parameters {
            return Self {
                path_and_query_matching: Some(if config.ignore_path_and_query_case {
                    url.to_lowercase()
                } else {
                    url.clone()
                }),
                path_and_query: url,
                original: path_and_query_str.to_string(),
                skipped_query_params: None,
            };
        }

        let path_and_query: PathAndQuery = match url.parse() {
            Ok(p) => p,
            Err(err) => {
                log::error!("cannot parse url '{path_and_query_str}', cancel ignoring marketing query params: {err}");

                return Self {
                    path_and_query_matching: Some(if config.ignore_path_and_query_case {
                        url.to_lowercase()
                    } else {
                        url.clone()
                    }),
                    path_and_query: url,
                    original: path_and_query_str.to_string(),
                    skipped_query_params: None,
                };
            }
        };

        let mut new_path_and_query = path_and_query.path().to_string();
        let mut skipped_query_params = "".to_string();

        if let Some(query) = path_and_query.query() {
            let mut query_string = "".to_string();

            if config.ignore_all_query_parameters {
                skipped_query_params = query.to_string();
            } else {
                let hash_query: LinkedHashMap<String, String> = parse_query(query.as_bytes()).into_owned().collect();
                let mut keys = hash_query.keys().cloned().collect::<Vec<String>>();

                if config.ignore_query_param_order {
                    keys.sort();
                }

                for key in &keys {
                    let value = hash_query.get(key).unwrap();
                    let mut query_param = "".to_string();

                    query_param.push_str(&utf8_percent_encode(key, QUERY_ENCODE_SET).to_string());

                    if !value.is_empty() {
                        query_param.push('=');
                        query_param.push_str(&utf8_percent_encode(value, QUERY_ENCODE_SET).to_string());
                    }

                    if config.marketing_query_params.contains(key) {
                        if !skipped_query_params.is_empty() {
                            skipped_query_params.push('&')
                        }

                        skipped_query_params.push_str(query_param.as_str())
                    } else {
                        if !query_string.is_empty() {
                            query_string.push('&');
                        }

                        query_string.push_str(query_param.as_str())
                    }
                }
            }

            if !query_string.is_empty() {
                new_path_and_query.push('?');
                new_path_and_query.push_str(query_string.as_str());
            }
        }

        Self {
            path_and_query_matching: Some(if config.ignore_path_and_query_case {
                new_path_and_query.to_lowercase()
            } else {
                new_path_and_query.clone()
            }),
            path_and_query: new_path_and_query,
            original: path_and_query_str.to_string(),
            skipped_query_params: if config.pass_marketing_query_params_to_target && !skipped_query_params.is_empty() {
                Some(skipped_query_params)
            } else {
                None
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use http::uri::PathAndQuery;

    use crate::http::query::sanitize_url;

    fn test_url(path: &str) {
        let sanitized = sanitize_url(path);
        let url = sanitized.parse::<PathAndQuery>();

        assert!(url.is_ok());
    }

    #[test]
    fn test_url_1() {
        test_url(
            "/npoplayer.html?tx_eonpo_npoplayer%5Bmid%5D=WO_EO_16582885&tx_eonpo_npoplayer%5Bhash%5D=45a69ca57ac8eee5025d45f06f9910f85fd9a0db814c590fg560293d<549880f&tx_eonpo_npoplayer%5Boverlay%5D=https%3A%2F%2Fblauwbloed.eo.nl%2Ffileadmin%2Fbestanden-2016%2Fuser_upload%2F2021-07%2FKoninklijk_gezin_fotosessie_zomer_2021.jpg&tx_eonpo_npoplayer%5Bhasadconsent%5D=0&tx_eonpo_npoplayer%5Breferralurl%5D=https%3A%2F%2Fblauwbloed.eo.nl%2Fartikel%2F2021%2F07%2Fkijk-de-eerste-foto-van-de-fotosessie-van-de-oranjes&tx_eonpo_npoplayer%5BsterSiteId%5D=blauwbloed&tx_eonpo_npoplayer%5BsterIdentifier%5D=blauwbloed-ios-smartphone&tx_eonpo_npoplayer%5BatinternetSiteId%5D=25&tx_eonpo_npoplayer%5BatinternetUserId%5D=287dbe14-d677-4b9b-8eeb-ecb389349db1&tx_eonpo_npoplayer%5BatinternetUserIdCookieDuration%5D=394",
        );
    }

    #[test]
    fn test_url_2() {
        test_url(
            "/fileadmin/bestanden-2016/_processed_/5/5/csm_Echte_vriendschap_Vanaf_de_eerste_dag_van_hun_studie_zijn_Inge_en_Julia_vrjendinnep_2_8260<c0281.jtg",
        );
    }
}
