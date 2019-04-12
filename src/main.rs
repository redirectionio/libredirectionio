pub mod router;

use std::fs;

fn main() {
    println!("Create router");
    let main_router = create_test_router(false);
    println!("Router created");
    println!("Match rule");
    let rule = main_router
        .match_rule("http://www.yolo.com/documentation/developer-documentation/getting-started-installing-the-agent".to_string());

    println!("Rules matched {:?}", rule);

    if rule.is_some() {
        let redirect = router::MainRouter::get_redirect(
            rule.unwrap(),
            "http://www.yolo.com/documentation/developer-documentation/getting-started-installing-the-agent".to_string(),
        );

        println!("Redirect to {}", redirect);
    }
}

fn create_test_router(cache: bool) -> router::MainRouter {
    let data = fs::read_to_string(
        "/home/joelwurtz/Archive/Redirection/redirection.io/clients/libredirectionio/rules.json",
    )
    .expect("Unable to read file");
    let deserialized: router::api::ApiAgentRuleResponse = serde_json::from_str(&data).unwrap();
    let rules_data = serde_json::to_string(&deserialized.rules).expect("Cannot serialize rules");

    return router::MainRouter::new_from_data(rules_data, cache);
}
