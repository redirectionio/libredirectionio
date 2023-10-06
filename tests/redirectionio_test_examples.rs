extern crate redirectionio;

#[rustfmt::skip]
mod generated_tests {
    use redirectionio::api::{TestExamplesInput, TestExamplesOutput};
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

        let test_examples = TestExamplesOutput::create_result_without_project(test_examples_input);

        let json_out_expected = std::fs::read_to_string(format!("tests/test_examples/{}.out.json", name)).unwrap();
        let json_out = json_encode(&test_examples).unwrap();

        if env::var("RIO_UPDATE_FIXTURES").is_ok() {
            std::fs::write(format!("tests/test_examples/{}.out.json", name), &json_out).unwrap();
            return;
        }

        if json_out != json_out_expected {
            std::fs::write(format!("tests/test_examples/{}.out.current.json", name), &json_out).unwrap();
        }

        assert_eq!(json_out, json_out_expected, "check for tests/test_examples/{}.out.current.json", name);
    }
}
