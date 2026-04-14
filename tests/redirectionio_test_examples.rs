extern crate redirectionio;

#[rustfmt::skip]
mod generated_tests {
    use redirectionio::action::RunExample;
    use redirectionio::api::{TestExamplesInput, Rule};
    use redirectionio::router::Router;
    use serde_json::{from_str as json_decode, to_string_pretty as json_encode};
    use std::env;

    #[test]
    fn test_examples_configuration_log_off() {
        do_test("configuration_log_off");
    }

    #[test]
    fn test_examples_configuration_reset_on() {
        do_test("configuration_reset_on");
    }

    #[test]
    fn test_examples_filter_html() {
        do_test("filter_html");
    }

    #[test]
    fn test_examples_header_add() {
        do_test("header_add");
    }

    #[test]
    fn test_examples_header_override() {
        do_test("header_override");
    }

    #[test]
    fn test_examples_header_remove() {
        do_test("header_remove");
    }

    #[test]
    fn test_examples_header_replace() {
        do_test("header_replace");
    }

    #[test]
    fn test_examples_must_match_false_broken() {
        do_test("must_match_false_broken");
    }

    #[test]
    fn test_examples_must_match_true_broken() {
        do_test("must_match_true_broken");
    }

    #[test]
    fn test_examples_no_examples() {
        do_test("no_examples");
    }

    #[test]
    fn test_examples_no_rules() {
        do_test("no_rules");
    }

    #[test]
    fn test_examples_one_rule_one_example() {
        do_test("one_rule_one_example");
    }

    fn do_test(name: &str) {
        let json_in = std::fs::read_to_string(format!("tests/test_examples/{}.in.json", name)).unwrap();
        let test_examples_input: TestExamplesInput = json_decode(&json_in).unwrap();

        let mut router = Router::<Rule>::from_config(test_examples_input.router_config.clone());

        for rule in test_examples_input.rules.iter() {
            router.insert(rule.clone());
        }

        let mut results = Vec::new();

        for rule in &test_examples_input.rules {
            if let Some(examples) = rule.examples.as_ref() {
                for example in examples {
                    let mut run_example = RunExample::new(&router, example).unwrap();
                    run_example.request.created_at = None;

                    results.push(run_example);
                }
            }
        }

        let json_out_expected = std::fs::read_to_string(format!("tests/test_examples/{}.out.json", name)).unwrap();
        let json_out = json_encode(&results).unwrap();

        if env::var("RIO_UPDATE_FIXTURES").is_ok() {
            std::fs::write(format!("tests/test_examples/{}.out.json", name), &json_out).unwrap();
            return;
        }

        if json_out != json_out_expected {
            std::fs::write(format!("tests/test_examples/{}.out.current.json", name), &json_out).unwrap();
        }

        assert_eq!(
            json_out.parse::<serde_json::Value>().unwrap(),
            json_out_expected.parse::<serde_json::Value>().unwrap(),
            "check for tests/test_examples/{}.out.current.json",
            name
        );
    }
}
