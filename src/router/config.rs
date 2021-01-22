pub struct RouterConfig {
    ignore_case: bool,
    ignore_marketing_query_params: bool,
    marketing_query_params: Vec<String>,
    pass_marketing_query_params_to_target: bool,
}

impl Default for RouterConfig {
    fn default() -> Self {
        RouterConfig {
            ignore_case: false,
            ignore_marketing_query_params: true,
            marketing_query_params: vec![
                "utm_source".to_string(),
                "utm_medium".to_string(),
                "utm_campaign".to_string(),
                "utm_term".to_string(),
                "utm_content".to_string(),
            ],
            pass_marketing_query_params_to_target: true,
        }
    }
}
