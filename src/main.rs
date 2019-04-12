pub mod router;

fn main() {
    println!("Create router");
    let main_router = router::create_test_router(false);
    println!("Router created");
    println!("Match rule");
    let rule = main_router.match_rule("https://www.ouibus.com/routes/aadouvresbb".to_string());

    if rule.is_some() {
        let redirect = router::MainRouter::get_redirect(
            rule.unwrap(),
            "https://www.ouibus.com/routes/aadouvresbb".to_string(),
        );

        println!("Redirect to {}", redirect);
    }

    println!("Rules matched {:?}", rule);
}
