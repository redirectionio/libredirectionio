use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::Hash;

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

impl Hash for RouterConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ignore_host_case.hash(state);
        self.ignore_header_case.hash(state);
        self.ignore_path_and_query_case.hash(state);
        self.ignore_marketing_query_params.hash(state);
        self.pass_marketing_query_params_to_target.hash(state);
        self.always_match_any_host.hash(state);

        // order hash set to make sure it's always the same
        let mut marketing_query_params: Vec<String> = self.marketing_query_params.iter().cloned().collect();
        marketing_query_params.sort();

        marketing_query_params.hash(state);
    }
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
