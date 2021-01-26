use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterConfig {
    pub ignore_host_case: bool,
    pub ignore_header_case: bool,
    pub ignore_path_and_query_case: bool,
    pub ignore_marketing_query_params: bool,
    pub marketing_query_params: HashSet<String>,
    pub pass_marketing_query_params_to_target: bool,
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
        }
    }
}
