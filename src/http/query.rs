use crate::router::RouterConfig;
use http::uri::PathAndQuery;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use url::form_urlencoded::parse as parse_query;

const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct PathAndQueryWithSkipped {
    pub path_and_query: String,
    pub path_and_query_matching: Option<String>,
    pub skipped_query_params: Option<String>,
    pub original: String,
}

fn sanitize_url(path_and_query_str: &str) -> String {
    utf8_percent_encode(path_and_query_str, QUERY_ENCODE_SET).to_string()
}

impl PathAndQueryWithSkipped {
    pub fn from_static(path_and_query_str: &str) -> Self {
        Self {
            path_and_query: path_and_query_str.to_string(),
            path_and_query_matching: Some(path_and_query_str.to_string()),
            original: path_and_query_str.to_string(),
            skipped_query_params: None,
        }
    }

    pub fn from_config(config: &RouterConfig, path_and_query_str: &str) -> Self {
        let url = sanitize_url(path_and_query_str);

        if !config.ignore_marketing_query_params {
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
                log::error!("cannot parse url {}, don't ignore markerting query params: {}", url, err);

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
            let hash_query: BTreeMap<_, _> = parse_query(query.as_bytes()).into_owned().collect();
            let mut query_string = "".to_string();

            for (key, value) in &hash_query {
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
    use crate::http::query::sanitize_url;
    use http::uri::PathAndQuery;

    fn test_url(path: &str) {
        let sanitized = sanitize_url(path);
        let url = sanitized.parse::<PathAndQuery>();

        println!("{:#?}", url);

        assert!(url.is_ok());
    }

    #[test]
    fn test_url_1() {
        test_url("/npoplayer.html?tx_eonpo_npoplayer%5Bmid%5D=WO_EO_16582885&tx_eonpo_npoplayer%5Bhash%5D=45a69ca57ac8eee5025d45f06f9910f85fd9a0db814c590fg560293d<549880f&tx_eonpo_npoplayer%5Boverlay%5D=https%3A%2F%2Fblauwbloed.eo.nl%2Ffileadmin%2Fbestanden-2016%2Fuser_upload%2F2021-07%2FKoninklijk_gezin_fotosessie_zomer_2021.jpg&tx_eonpo_npoplayer%5Bhasadconsent%5D=0&tx_eonpo_npoplayer%5Breferralurl%5D=https%3A%2F%2Fblauwbloed.eo.nl%2Fartikel%2F2021%2F07%2Fkijk-de-eerste-foto-van-de-fotosessie-van-de-oranjes&tx_eonpo_npoplayer%5BsterSiteId%5D=blauwbloed&tx_eonpo_npoplayer%5BsterIdentifier%5D=blauwbloed-ios-smartphone&tx_eonpo_npoplayer%5BatinternetSiteId%5D=25&tx_eonpo_npoplayer%5BatinternetUserId%5D=287dbe14-d677-4b9b-8eeb-ecb389349db1&tx_eonpo_npoplayer%5BatinternetUserIdCookieDuration%5D=394");
    }

    #[test]
    fn test_url_2() {
        test_url("/fileadmin/bestanden-2016/_processed_/5/5/csm_Echte_vriendschap_Vanaf_de_eerste_dag_van_hun_studie_zijn_Inge_en_Julia_vrjendinnep_2_8260<c0281.jtg");
    }
}
