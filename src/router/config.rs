use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterConfig {
    #[serde(default = "default_as_false")]
    pub ignore_host_case: bool,
    #[serde(default = "default_as_false")]
    pub ignore_header_case: bool,
    #[serde(default = "default_as_false")]
    pub ignore_path_and_query_case: bool,
    #[serde(default)]
    pub ignore_marketing_query_params: bool,
    #[serde(default = "default_marketing_parameters")]
    pub marketing_query_params: HashSet<String>,
    #[serde(default)]
    pub pass_marketing_query_params_to_target: bool,
    #[serde(default = "default_as_false")]
    pub always_match_any_host: bool,
}

fn default_as_false() -> bool {
    false
}

fn default_marketing_parameters() -> HashSet<String> {
    let mut parameters = HashSet::new();

    parameters.insert("utm_source".to_string());
    parameters.insert("utm_medium".to_string());
    parameters.insert("utm_campaign".to_string());
    parameters.insert("utm_term".to_string());
    parameters.insert("utm_content".to_string());

    parameters
}

impl Default for RouterConfig {
    fn default() -> Self {
        let mut parameters = HashSet::new();

        parameters.insert("utm_source".to_string());
        parameters.insert("utm_medium".to_string());
        parameters.insert("utm_campaign".to_string());
        parameters.insert("utm_term".to_string());
        parameters.insert("utm_content".to_string());

        RouterConfig {
            ignore_host_case: false,
            ignore_header_case: false,
            ignore_path_and_query_case: false,
            ignore_marketing_query_params: true,
            marketing_query_params: parameters,
            pass_marketing_query_params_to_target: true,
            always_match_any_host: true,
        }
    }
}
