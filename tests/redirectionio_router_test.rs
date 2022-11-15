extern crate redirectionio;

#[rustfmt::skip]
mod generated_tests {

use redirectionio::router::{Router, RouterConfig, Trace};
use redirectionio::api::Rule;
use redirectionio::http::{Request, Header, PathAndQueryWithSkipped};
use redirectionio::action::Action;


fn setup_00_common_rules() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"simple-foobar-rule","rank":0,"source":{"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_00_common_rules_1() {
    let router = setup_00_common_rules();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_00_common_rules_2() {
    let router = setup_00_common_rules();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo2"#), r#"/foo2"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_01_straight_rule_match() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"any-host-path","rank":0,"source":{"path":"/foo"},"status_code":301,"target":"/any-host--path-only"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"any-host-path-query","rank":0,"source":{"path":"/foo","query":"bar=baz"},"status_code":301,"target":"/any-host--path-query"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"any-host-query-only","rank":0,"source":{"path":"/","query":"bar=baz"},"status_code":301,"target":"/any-host--query-only"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    let route_4: Rule = serde_json::from_str(r#"{"id":"empty","rank":0,"source":{"path":"/"},"status_code":301,"target":"/empty"}"#).expect("cannot deserialize");
    router.insert(route_4.into_route(&router.config));

    let route_5: Rule = serde_json::from_str(r#"{"id":"example-net-host-only","rank":0,"source":{"host":"example.net","path":"/"},"status_code":301,"target":"/example.net--host-only"}"#).expect("cannot deserialize");
    router.insert(route_5.into_route(&router.config));

    let route_6: Rule = serde_json::from_str(r#"{"id":"example-net-host-path","rank":0,"source":{"host":"example.net","path":"/foo"},"status_code":301,"target":"/example.net--host-path-only"}"#).expect("cannot deserialize");
    router.insert(route_6.into_route(&router.config));

    let route_7: Rule = serde_json::from_str(r#"{"id":"example-net-host-path-query","rank":0,"source":{"host":"example.net","path":"/foo","query":"bar=baz"},"status_code":301,"target":"/example.net--host-path-query"}"#).expect("cannot deserialize");
    router.insert(route_7.into_route(&router.config));

    let route_8: Rule = serde_json::from_str(r#"{"id":"host","rank":0,"source":{"host":"example.org","path":"/"},"status_code":301,"target":"/example.org--host-only"}"#).expect("cannot deserialize");
    router.insert(route_8.into_route(&router.config));

    let route_9: Rule = serde_json::from_str(r#"{"id":"host-path-query","rank":0,"source":{"host":"example.org","path":"/foo","query":"bar=baz"},"status_code":301,"target":"/example.org--host-path-query"}"#).expect("cannot deserialize");
    router.insert(route_9.into_route(&router.config));

    let route_10: Rule = serde_json::from_str(r#"{"id":"host-with-path","rank":0,"source":{"host":"example.org","path":"/foo"},"status_code":301,"target":"/example.org--host-path-only"}"#).expect("cannot deserialize");
    router.insert(route_10.into_route(&router.config));

    let route_11: Rule = serde_json::from_str(r#"{"id":"host-with-query","rank":0,"source":{"host":"example.org","path":"/","query":"bar=baz"},"status_code":301,"target":"/example.org--host-query-only"}"#).expect("cannot deserialize");
    router.insert(route_11.into_route(&router.config));

    let route_12: Rule = serde_json::from_str(r#"{"id":"path-with-plus-sign","rank":0,"source":{"host":"www.domain.nl","path":"/zwart+janstraat"},"status_code":301,"target":"/plus-sign"}"#).expect("cannot deserialize");
    router.insert(route_12.into_route(&router.config));

    let route_13: Rule = serde_json::from_str(r#"{"id":"path-with-space-percent-encoded","rank":0,"source":{"host":"example.net","path":"/i%20have%20space"},"status_code":301,"target":"/space"}"#).expect("cannot deserialize");
    router.insert(route_13.into_route(&router.config));

    router
}


#[test]
fn test_01_straight_rule_match_1() {
    let router = setup_01_straight_rule_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/example.org--host-path-only"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_01_straight_rule_match_2() {
    let router = setup_01_straight_rule_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?bar=baz"#), r#"/foo?bar=baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/example.org--host-path-query"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_01_straight_rule_match_3() {
    let router = setup_01_straight_rule_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/?q"#), r#"/?q"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_01_straight_rule_match_4() {
    let router = setup_01_straight_rule_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/?"#), r#"/?"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/empty"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_01_straight_rule_match_5() {
    let router = setup_01_straight_rule_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"?bar2=baz"#), r#"?bar2=baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_01_straight_rule_match_6() {
    let router = setup_01_straight_rule_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?bar=baz"#), r#"/foo?bar=baz"#.to_string(),Some(r#"foobar.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/any-host--path-query"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_01_straight_rule_match_7() {
    let router = setup_01_straight_rule_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),Some(r#"example.net"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/example.net--host-path-only"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_01_straight_rule_match_8() {
    let router = setup_01_straight_rule_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?bar=baz"#), r#"/foo?bar=baz"#.to_string(),Some(r#"example.net"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/example.net--host-path-query"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_01_straight_rule_match_9() {
    let router = setup_01_straight_rule_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/i%20have%20space"#), r#"/i%20have%20space"#.to_string(),Some(r#"example.net"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/space"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_01_straight_rule_match_10() {
    let router = setup_01_straight_rule_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/i have space"#), r#"/i have space"#.to_string(),Some(r#"example.net"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/space"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_01_straight_rule_match_11() {
    let router = setup_01_straight_rule_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/zwart+janstraat"#), r#"/zwart+janstraat"#.to_string(),Some(r#"www.domain.nl"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/plus-sign"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_03_priority_match() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"complex-example","rank":10,"source":{"path":"/foo"},"status_code":301,"target":"/complex-example-org"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"complex-example-net","rank":10,"source":{"path":"/foo"},"status_code":301,"target":"/complex-example-net"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"straight-any-host","rank":1,"source":{"path":"/foo"},"status_code":301,"target":"/straight-any-host"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    let route_4: Rule = serde_json::from_str(r#"{"id":"straight-example-net","rank":20,"source":{"host":"example.net","path":"/foo"},"status_code":301,"target":"/straight-example-net"}"#).expect("cannot deserialize");
    router.insert(route_4.into_route(&router.config));

    let route_5: Rule = serde_json::from_str(r#"{"id":"straigth-example","rank":1,"source":{"host":"example.org","path":"/foo"},"status_code":301,"target":"/straight-example-org"}"#).expect("cannot deserialize");
    router.insert(route_5.into_route(&router.config));

    let route_6: Rule = serde_json::from_str(r#"{"id":"straigth-example-same-rank-but-after","rank":1,"source":{"host":"example.fr","path":"/foo"},"status_code":301,"target":"/straight-example-fr"}"#).expect("cannot deserialize");
    router.insert(route_6.into_route(&router.config));

    router
}


#[test]
fn test_03_priority_match_1() {
    let router = setup_03_priority_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/straight-example-org"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_03_priority_match_2() {
    let router = setup_03_priority_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),Some(r#"example.com"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/straight-any-host"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_03_priority_match_3() {
    let router = setup_03_priority_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),Some(r#"example.net"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/straight-example-net"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_03_priority_match_4() {
    let router = setup_03_priority_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),Some(r#"example.fr"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/straight-example-fr"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_04_rfc3986_relative_references() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"doublepathSource","rank":0,"source":{"path":"//xyz"},"status_code":301,"target":"/xyz"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"doublepathSourceWithHost","rank":0,"source":{"host":"yolo.com","path":"//doubledragon"},"status_code":301,"target":"/simpledragon"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"doublepathTarget","rank":0,"source":{"path":"/source"},"status_code":301,"target":"//target"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    router
}


#[test]
fn test_04_rfc3986_relative_references_1() {
    let router = setup_04_rfc3986_relative_references();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"//xyz"#), r#"//xyz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/xyz"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_04_rfc3986_relative_references_2() {
    let router = setup_04_rfc3986_relative_references();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/xyz"#), r#"/xyz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_04_rfc3986_relative_references_3() {
    let router = setup_04_rfc3986_relative_references();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"//target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_04_rfc3986_relative_references_4() {
    let router = setup_04_rfc3986_relative_references();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"//doubledragon"#), r#"//doubledragon"#.to_string(),Some(r#"yolo.com"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/simpledragon"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_05_query_parameters_order() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-inverted-with-query-parameters","rank":0,"source":{"path":"/foo","query":"c=c&b=b"},"status_code":302,"target":"/bar-inverted"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-with-query-parameters","rank":0,"source":{"path":"/foo","query":"a=a&b=b"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_05_query_parameters_order_1() {
    let router = setup_05_query_parameters_order();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?a=a&b=b"#), r#"/foo?a=a&b=b"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_05_query_parameters_order_2() {
    let router = setup_05_query_parameters_order();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?b=b&a=a"#), r#"/foo?b=b&a=a"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_05_query_parameters_order_3() {
    let router = setup_05_query_parameters_order();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?a=a&b=b&c=c"#), r#"/foo?a=a&b=b&c=c"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_05_query_parameters_order_4() {
    let router = setup_05_query_parameters_order();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?b=b&c=c"#), r#"/foo?b=b&c=c"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar-inverted"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_05_query_parameters_order_5() {
    let router = setup_05_query_parameters_order();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?c=c&b=b"#), r#"/foo?c=c&b=b"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar-inverted"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_06_emojis() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"simple-emoji-rule","rank":0,"source":{"path":"/🍕"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_06_emojis_1() {
    let router = setup_06_emojis();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/🍕"#), r#"/🍕"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_disable_log() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"action-serve-robotxt","log_override":false,"rank":0,"reset":true,"source":{"path":"/no-log"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_disable_log_1() {
    let router = setup_action_disable_log();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/no-log"#), r#"/no-log"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), false);
}

#[test]
fn test_action_disable_log_2() {
    let router = setup_action_disable_log();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/no-log-2"#), r#"/no-log-2"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_action_filter_header_add() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"header_filters":[{"action":"add","header":"X-Foo","value":"foo2"}],"id":"action-header-add","rank":2,"source":{"path":"/foo"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_filter_header_add_1() {
    let router = setup_action_filter_header_add();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), true);
    let mut response_headers = Vec::new();

    response_headers.push(Header {
        name: r#"X-Foo"#.to_string(),
        value: r#"foo1"#.to_string(),
    });

    let filtered_headers = action.filter_headers(response_headers, response_status_code, false, None);
    let header_map = Header::create_header_map(filtered_headers);

    let value = header_map.get(r#"X-Foo"#);

    assert!(value.is_some());
    assert_eq!(value.unwrap(), r#"foo1"#);

}

#[test]
fn test_action_filter_header_add_2() {
    let router = setup_action_filter_header_add();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), true);
    let response_headers = Vec::new();

    let filtered_headers = action.filter_headers(response_headers, response_status_code, false, None);
    let header_map = Header::create_header_map(filtered_headers);

    let value = header_map.get(r#"X-Foo"#);

    assert!(value.is_some());
    assert_eq!(value.unwrap(), r#"foo2"#);

}


fn setup_action_filter_header_override() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"header_filters":[{"action":"override","header":"X-Foo","value":"foo2"}],"id":"action-header-override","rank":2,"source":{"path":"/foo"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_filter_header_override_1() {
    let router = setup_action_filter_header_override();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), true);
    let mut response_headers = Vec::new();

    response_headers.push(Header {
        name: r#"X-Foo"#.to_string(),
        value: r#"foo1"#.to_string(),
    });

    let filtered_headers = action.filter_headers(response_headers, response_status_code, false, None);
    let header_map = Header::create_header_map(filtered_headers);

    let value = header_map.get(r#"X-Foo"#);

    assert!(value.is_some());
    assert_eq!(value.unwrap(), r#"foo2"#);

    let value = header_map.get(r#"X-Bar"#);

    assert!(value.is_none());

}

#[test]
fn test_action_filter_header_override_2() {
    let router = setup_action_filter_header_override();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), true);
    let mut response_headers = Vec::new();

    response_headers.push(Header {
        name: r#"X-foo"#.to_string(),
        value: r#"foo1"#.to_string(),
    });

    let filtered_headers = action.filter_headers(response_headers, response_status_code, false, None);
    let header_map = Header::create_header_map(filtered_headers);

    let value = header_map.get(r#"X-foo"#);

    assert!(value.is_some());
    assert_eq!(value.unwrap(), r#"foo2"#);

    let value = header_map.get(r#"X-Bar"#);

    assert!(value.is_none());

}


fn setup_action_redirection() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"action-gone-404","rank":0,"source":{"path":"/foo"},"status_code":404}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"action-gone-410","rank":0,"source":{"path":"/foo"},"status_code":410}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"action-redirection-301","rank":0,"source":{"path":"/foo"},"status_code":301,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    let route_4: Rule = serde_json::from_str(r#"{"id":"action-redirection-302","rank":0,"source":{"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_4.into_route(&router.config));

    let route_5: Rule = serde_json::from_str(r#"{"id":"action-redirection-307","rank":0,"source":{"path":"/foo"},"status_code":307,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_5.into_route(&router.config));

    let route_6: Rule = serde_json::from_str(r#"{"id":"action-redirection-308","rank":0,"source":{"path":"/foo"},"status_code":308,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_6.into_route(&router.config));

    router
}



fn setup_action_reset() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"header_filters":[{"action":"add","header":"X-Bar","value":"bar"}],"id":"action-after","rank":0,"source":{"path":"/foo"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"header_filters":[{"action":"add","header":"X-Foo","value":"foo"}],"id":"action-before","rank":2,"source":{"path":"/foo"}}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"action-stop","rank":1,"reset":true,"source":{"path":"/foo"}}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    router
}


#[test]
fn test_action_reset_1() {
    let router = setup_action_reset();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), true);
    let response_headers = Vec::new();

    let filtered_headers = action.filter_headers(response_headers, response_status_code, false, None);
    let header_map = Header::create_header_map(filtered_headers);

    let value = header_map.get(r#"X-Bar"#);

    assert!(value.is_some());
    assert_eq!(value.unwrap(), r#"bar"#);

    let value = header_map.get(r#"X-Foo"#);

    assert!(value.is_none());

}


fn setup_action_robots_txt() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"replace_text","content":"User-Agent: *"}],"id":"action-serve-robotxt","rank":0,"source":{"path":"/robots.txt"},"status_code":200}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_robots_txt_1() {
    let router = setup_action_robots_txt();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/robots.txt"#), r#"/robots.txt"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 200);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#""#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"User-Agent: *"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_seo_override_meta_author() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[name=\"author\"]","element_tree":["html","head"],"value":"<meta name=\"author\" content=\"Author name\" />"},{"action":"replace","css_selector":"meta[name=\"author\"]","element_tree":["html","head","meta"],"value":"<meta name=\"author\" content=\"Author name\" />"}],"id":"override-meta-author-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_seo_override_meta_author_1() {
    let router = setup_action_seo_override_meta_author();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta name="author" content="Author name" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_author_2() {
    let router = setup_action_seo_override_meta_author();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="author" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_author_3() {
    let router = setup_action_seo_override_meta_author();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="author" content="Old Author name" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_author_4() {
    let router = setup_action_seo_override_meta_author();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="author" /><meta name="author" content="Old Author name" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /><meta name="author" content="Author name" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_author_5() {
    let router = setup_action_seo_override_meta_author();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="author" content="Old first Author name" /><meta name="author" content="Old second Author name" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /><meta name="author" content="Author name" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_author_6() {
    let router = setup_action_seo_override_meta_author();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta><meta name="author" content="Author name" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_author_7() {
    let router = setup_action_seo_override_meta_author();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="author"></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_author_8() {
    let router = setup_action_seo_override_meta_author();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="author" content="Old Author name"></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="author" content="Author name" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_seo_override_meta_description() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[name=\"description\"]","element_tree":["html","head"],"value":"<meta name=\"description\" content=\"New Description\" />"},{"action":"replace","css_selector":"meta[name=\"description\"]","element_tree":["html","head","meta"],"value":"<meta name=\"description\" content=\"New Description\" />"}],"id":"override-meta-description-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_seo_override_meta_description_1() {
    let router = setup_action_seo_override_meta_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta name="description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_description_2() {
    let router = setup_action_seo_override_meta_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="description" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_description_3() {
    let router = setup_action_seo_override_meta_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="description" content="Old Description" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_description_4() {
    let router = setup_action_seo_override_meta_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta><meta name="description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_description_5() {
    let router = setup_action_seo_override_meta_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="description"></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_description_6() {
    let router = setup_action_seo_override_meta_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="description" content="Old Description"></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_seo_override_meta_keywords() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[name=\"keywords\"]","element_tree":["html","head"],"value":"<meta name=\"keywords\" content=\"some, keywords, here\" />"},{"action":"replace","css_selector":"meta[name=\"keywords\"]","element_tree":["html","head","meta"],"value":"<meta name=\"keywords\" content=\"some, keywords, here\" />"}],"id":"override-meta-keywords-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_seo_override_meta_keywords_1() {
    let router = setup_action_seo_override_meta_keywords();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta name="keywords" content="some, keywords, here" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_keywords_2() {
    let router = setup_action_seo_override_meta_keywords();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="keywords" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="keywords" content="some, keywords, here" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_keywords_3() {
    let router = setup_action_seo_override_meta_keywords();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="keywords" content="these, were, old, keywords" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="keywords" content="some, keywords, here" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_keywords_4() {
    let router = setup_action_seo_override_meta_keywords();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta><meta name="keywords" content="some, keywords, here" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_keywords_5() {
    let router = setup_action_seo_override_meta_keywords();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="keywords"></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="keywords" content="some, keywords, here" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_keywords_6() {
    let router = setup_action_seo_override_meta_keywords();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta name="keywords" content="these, were, old, keywords"></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta name="keywords" content="some, keywords, here" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_meta_keywords_7() {
    let router = setup_action_seo_override_meta_keywords();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><link rel="shortcut icon" href="/favicon.ico"></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><link rel="shortcut icon" href="/favicon.ico"><meta name="keywords" content="some, keywords, here" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_seo_override_og_description() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:description\"]","element_tree":["html","head"],"value":"<meta property=\"og:description\" content=\"🍕🍕 Pizza rapido 🍕🍕\" />"},{"action":"replace","css_selector":"meta[property=\"og:description\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:description\" content=\"🍕🍕 Pizza rapido 🍕🍕\" />"}],"id":"override-og-description-emoji-rule","rank":0,"source":{"host":"","path":"/pizza-rapido","query":""}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:description\"]","element_tree":["html","head"],"value":"<meta property=\"og:description\" content=\"New Description\" />"},{"action":"replace","css_selector":"meta[property=\"og:description\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:description\" content=\"New Description\" />"}],"id":"override-og-description-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_action_seo_override_og_description_1() {
    let router = setup_action_seo_override_og_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><description>Old description</description><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><description>Old description</description><meta /><meta property="og:description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_description_2() {
    let router = setup_action_seo_override_og_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_description_3() {
    let router = setup_action_seo_override_og_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><description>Old description</description><meta /><meta property="og:description" content="Old Description" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><description>Old description</description><meta /><meta property="og:description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_description_4() {
    let router = setup_action_seo_override_og_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><description>Old description</description><meta><meta property="og:description" content="Old Description" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><description>Old description</description><meta><meta property="og:description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_description_5() {
    let router = setup_action_seo_override_og_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><description>Old description</description><meta></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><description>Old description</description><meta><meta property="og:description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_description_6() {
    let router = setup_action_seo_override_og_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><description>Old description</description><meta property="no-closing"><meta property="og:description" content="Old Description" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><description>Old description</description><meta property="no-closing"><meta property="og:description" content="New Description" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_description_7() {
    let router = setup_action_seo_override_og_description();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/pizza-rapido"#), r#"/pizza-rapido"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta property="og:description" content="Pizza rapido" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta property="og:description" content="🍕🍕 Pizza rapido 🍕🍕" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_seo_override_og_image() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:image\"]","element_tree":["html","head"],"value":"<meta property=\"og:image\" content=\"/some-image.png\" />"},{"action":"replace","css_selector":"meta[property=\"og:image\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:image\" content=\"/some-image.png\" />"}],"id":"override-og-image-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_seo_override_og_image_1() {
    let router = setup_action_seo_override_og_image();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:image" content="/some-image.png" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_image_2() {
    let router = setup_action_seo_override_og_image();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta><meta property="og:image" content="/some-image.png" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_image_3() {
    let router = setup_action_seo_override_og_image();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /><meta property="og:image" content="/old-image.png" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:image" content="/some-image.png" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_image_4() {
    let router = setup_action_seo_override_og_image();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta property="no-closing"><meta property="og:image" content="/old-image.png" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta property="no-closing"><meta property="og:image" content="/some-image.png" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_seo_override_og_locale() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:locale\"]","element_tree":["html","head"],"value":"<meta property=\"og:locale\" content=\"fr_FR\" />"},{"action":"replace","css_selector":"meta[property=\"og:locale\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:locale\" content=\"fr_FR\" />"}],"id":"override-og-locale-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_seo_override_og_locale_1() {
    let router = setup_action_seo_override_og_locale();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:locale" content="fr_FR" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_locale_2() {
    let router = setup_action_seo_override_og_locale();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta><meta property="og:locale" content="fr_FR" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_locale_3() {
    let router = setup_action_seo_override_og_locale();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /><meta property="og:locale" content="en_GB" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:locale" content="fr_FR" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_locale_4() {
    let router = setup_action_seo_override_og_locale();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta property="no-closing"><meta property="og:locale" content="en_GB" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta property="no-closing"><meta property="og:locale" content="fr_FR" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_seo_override_og_site_name() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:site_name\"]","element_tree":["html","head"],"value":"<meta property=\"og:site_name\" content=\"redirection.io\" />"},{"action":"replace","css_selector":"meta[property=\"og:site_name\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:site_name\" content=\"redirection.io\" />"}],"id":"override-og-site_name-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_seo_override_og_site_name_1() {
    let router = setup_action_seo_override_og_site_name();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:site_name" content="redirection.io" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_site_name_2() {
    let router = setup_action_seo_override_og_site_name();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta><meta property="og:site_name" content="redirection.io" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_site_name_3() {
    let router = setup_action_seo_override_og_site_name();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /><meta property="og:site_name" content="JoliCode" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:site_name" content="redirection.io" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_site_name_4() {
    let router = setup_action_seo_override_og_site_name();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta property="no-closing"><meta property="og:site_name" content="JoliCode" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta property="no-closing"><meta property="og:site_name" content="redirection.io" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_seo_override_og_title() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:title\"]","element_tree":["html","head"],"value":"<meta property=\"og:title\" content=\"New Title\" />"},{"action":"replace","css_selector":"meta[property=\"og:title\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:title\" content=\"New Title\" />"}],"id":"override-og-title-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_seo_override_og_title_1() {
    let router = setup_action_seo_override_og_title();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><title>Old title</title><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><title>Old title</title><meta /><meta property="og:title" content="New Title" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_title_2() {
    let router = setup_action_seo_override_og_title();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:title" content="New Title" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_title_3() {
    let router = setup_action_seo_override_og_title();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><title>Old title</title><meta /><meta property="og:title" content="Old Title" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><title>Old title</title><meta /><meta property="og:title" content="New Title" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_title_4() {
    let router = setup_action_seo_override_og_title();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><title>Old title</title><meta property="no-closing"><meta property="og:title" content="Old Title" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><title>Old title</title><meta property="no-closing"><meta property="og:title" content="New Title" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_seo_override_og_type() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:type\"]","element_tree":["html","head"],"value":"<meta property=\"og:type\" content=\"website\" />"},{"action":"replace","css_selector":"meta[property=\"og:type\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:type\" content=\"website\" />"}],"id":"override-og-type-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_seo_override_og_type_1() {
    let router = setup_action_seo_override_og_type();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:type" content="website" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_type_2() {
    let router = setup_action_seo_override_og_type();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:type" content="website" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_type_3() {
    let router = setup_action_seo_override_og_type();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /><meta property="og:type" content="article" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:type" content="website" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_type_4() {
    let router = setup_action_seo_override_og_type();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta property="no-closing"><meta property="og:type" content="article" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta property="no-closing"><meta property="og:type" content="website" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_seo_override_og_url() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"meta[property=\"og:url\"]","element_tree":["html","head"],"value":"<meta property=\"og:url\" content=\"https://redirection.io/features\" />"},{"action":"replace","css_selector":"meta[property=\"og:url\"]","element_tree":["html","head","meta"],"value":"<meta property=\"og:url\" content=\"https://redirection.io/features\" />"}],"id":"override-og-url-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_seo_override_og_url_1() {
    let router = setup_action_seo_override_og_url();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><url>Old url</url><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><url>Old url</url><meta /><meta property="og:url" content="https://redirection.io/features" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_url_2() {
    let router = setup_action_seo_override_og_url();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><meta property="og:url" content="https://redirection.io/features" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_url_3() {
    let router = setup_action_seo_override_og_url();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><url>Old url</url><meta /><meta property="og:url" content="https://jolicode.com/" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><url>Old url</url><meta /><meta property="og:url" content="https://redirection.io/features" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_og_url_4() {
    let router = setup_action_seo_override_og_url();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><url>Old url</url><meta property="no-closing"><meta property="og:url" content="https://jolicode.com/" /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><url>Old url</url><meta property="no-closing"><meta property="og:url" content="https://redirection.io/features" /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_seo_override_title() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_child","css_selector":"title","element_tree":["html","head"],"value":"<title>New Title</title>"},{"action":"replace","css_selector":"","element_tree":["html","head","title"],"value":"<title>New Title</title>"}],"id":"override-title-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_seo_override_title_1() {
    let router = setup_action_seo_override_title();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><title>Old title</title><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><title>New Title</title><meta /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_title_2() {
    let router = setup_action_seo_override_title();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta /><title>New Title</title></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_title_3() {
    let router = setup_action_seo_override_title();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><title>Old title</title><meta></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><title>New Title</title><meta></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_seo_override_title_4() {
    let router = setup_action_seo_override_title();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><meta></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><meta><title>New Title</title></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_sitemap() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"replace_text","content":"<sitemapindex xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\"><sitemap><loc>https://redirection.io/sitemap_static.xml</loc></sitemap><sitemap><loc>https://redirection.io/features/sitemap/</loc></sitemap><sitemap><loc>https://redirection.io/news/sitemap/</loc></sitemap></sitemapindex>"}],"id":"action-sitemap","rank":0,"source":{"path":"/sitemap.xml"},"status_code":200}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_sitemap_1() {
    let router = setup_action_sitemap();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/sitemap.xml"#), r#"/sitemap.xml"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 200);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#""#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"><sitemap><loc>https://redirection.io/sitemap_static.xml</loc></sitemap><sitemap><loc>https://redirection.io/features/sitemap/</loc></sitemap><sitemap><loc>https://redirection.io/news/sitemap/</loc></sitemap></sitemapindex>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_stop() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"header_filters":[{"action":"add","header":"X-Bar","value":"bar"}],"id":"action-after","rank":0,"source":{"path":"/foo"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"header_filters":[{"action":"add","header":"X-Foo","value":"foo"}],"id":"action-before","rank":2,"source":{"path":"/foo"}}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"action-stop","rank":1,"source":{"path":"/foo"},"stop":true}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    router
}


#[test]
fn test_action_stop_1() {
    let router = setup_action_stop();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), true);
    let response_headers = Vec::new();

    let filtered_headers = action.filter_headers(response_headers, response_status_code, false, None);
    let header_map = Header::create_header_map(filtered_headers);

    let value = header_map.get(r#"X-Foo"#);

    assert!(value.is_some());
    assert_eq!(value.unwrap(), r#"foo"#);

    let value = header_map.get(r#"X-Bar"#);

    assert!(value.is_none());

}


fn setup_action_text_append() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"append_text","content":"new content"}],"id":"override-title-rule","rank":0,"source":{"host":"","path":"/source","query":""}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_text_append_1() {
    let router = setup_action_text_append();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"Old content"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"Old contentnew content"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_text_prepend() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"prepend_text","content":"new content"}],"id":"override-title-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_text_prepend_1() {
    let router = setup_action_text_prepend();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"Old content"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"new contentOld content"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_action_text_replace() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"replace_text","content":"new content"}],"id":"override-title-rule","rank":0,"source":{"path":"/source"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_action_text_replace_1() {
    let router = setup_action_text_replace();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"Old content"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"new content"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_action_text_replace_2() {
    let router = setup_action_text_replace();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#""#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"new content"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_ignore_path_case() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":true,"marketing_query_params":["test"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule","rank":0,"source":{"path":"/FOo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-marker","markers":[{"name":"marker","regex":"([A-Z]+?)"}],"rank":0,"source":{"path":"/marker/@marker"},"status_code":302,"target":"/marker-target"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_ignore_path_case_1() {
    let router = setup_ignore_path_case();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_ignore_path_case_2() {
    let router = setup_ignore_path_case();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/Foo"#), r#"/Foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_ignore_path_case_3() {
    let router = setup_ignore_path_case();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/FOO"#), r#"/FOO"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_ignore_path_case_4() {
    let router = setup_ignore_path_case();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/FOo"#), r#"/FOo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_ignore_path_case_5() {
    let router = setup_ignore_path_case();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/marker/test"#), r#"/marker/test"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/marker-target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_ignore_path_case_6() {
    let router = setup_ignore_path_case();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/marker/TEST"#), r#"/marker/TEST"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/marker-target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_ignore_path_case_7() {
    let router = setup_ignore_path_case();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo2"#), r#"/foo2"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_marker() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"foobar-rule","markers":[{"name":"marker","regex":"(?:.+?)"}],"rank":0,"source":{"path":"/foo/@marker"},"status_code":302,"target":"/bar/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-segfault-on-target","markers":[{"name":"marker","regex":"(?:([\\p{Ll}\\p{Lu}\\p{Lt}0-9]|%[0-9A-Z]{2})+?)"}],"rank":0,"source":{"path":"/monthly-tides/North%20Carolina-North%20Shore/@marker"},"status_code":301,"target":"https://www.usharbors.com/harbor/western-pacific-coast/@marker"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"transformerRule","markers":[{"name":"marker","regex":"(?:.+?)","transformers":[{"options":null,"type":"dasherize"},{"options":null,"type":"uppercase"}]}],"rank":0,"source":{"path":"/a/@marker"},"status_code":302,"target":"/a/@marker"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    router
}


#[test]
fn test_marker_1() {
    let router = setup_marker();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo/test"#), r#"/foo/test"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar/test"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_2() {
    let router = setup_marker();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo2"#), r#"/foo2"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_3() {
    let router = setup_marker();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/a/test"#), r#"/a/test"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/a/TEST"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_4() {
    let router = setup_marker();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/a/test_test"#), r#"/a/test_test"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/a/TEST-TEST"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_5() {
    let router = setup_marker();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/monthly-tides/North%20Carolina-North%20Shore/test"#), r#"/monthly-tides/North%20Carolina-North%20Shore/test"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"https://www.usharbors.com/harbor/western-pacific-coast/test"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_case() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":false,"ignore_path_and_query_case":true,"marketing_query_params":[],"pass_marketing_query_params_to_target":false}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"anything-rule","markers":[{"name":"anything","regex":".*"}],"rank":0,"source":{"path":"/ExampleTest/@anything"},"status_code":301,"target":"/target/@anything"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_case_1() {
    let router = setup_marker_case();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/ExampleTest/UPPERCASE"#), r#"/ExampleTest/UPPERCASE"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target/UPPERCASE"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_case_2() {
    let router = setup_marker_case();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/ExampleTest/UpErCase"#), r#"/ExampleTest/UpErCase"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target/UpErCase"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_in_body_filter() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"body_filters":[{"action":"replace","css_selector":"","element_tree":["html","head","title"],"value":"<title>@marker</title>"}],"id":"marker-in-header-filter","markers":[{"name":"marker","regex":"(?:.+?)"}],"rank":0,"source":{"path":"/@marker"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_in_body_filter_1() {
    let router = setup_marker_in_body_filter();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    let body_filter_opt = action.create_filter_body(response_status_code, &[]);
    assert_eq!(body_filter_opt.is_some(), true);

    let mut body_filter = body_filter_opt.unwrap();
    let mut new_body = body_filter.filter(r#"<html><head><title>Old title</title><meta /></head></html>"#.as_bytes().to_vec(), None);
    new_body.extend(body_filter.end(None));
    assert_eq!(new_body, r#"<html><head><title>source</title><meta /></head></html>"#.as_bytes().to_vec());
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_in_header_filter() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"header_filters":[{"action":"replace","header":"X-Test","value":"@marker"}],"id":"marker-in-body-filter","markers":[{"name":"marker","regex":"(?:.+?)"}],"rank":0,"source":{"path":"/@marker"}}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_in_header_filter_1() {
    let router = setup_marker_in_header_filter();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), true);
    let mut response_headers = Vec::new();

    response_headers.push(Header {
        name: r#"X-Test"#.to_string(),
        value: r#"foo"#.to_string(),
    });

    let filtered_headers = action.filter_headers(response_headers, response_status_code, false, None);
    let header_map = Header::create_header_map(filtered_headers);

    let value = header_map.get(r#"X-Test"#);

    assert!(value.is_some());
    assert_eq!(value.unwrap(), r#"source"#);

}


fn setup_marker_in_host() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"marker-in-host-rule","markers":[{"name":"marker","regex":"(?:.+?)"}],"rank":0,"source":{"host":"@marker.test.com","path":"/"},"status_code":302,"target":"https://@marker.test.io"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_in_host_1() {
    let router = setup_marker_in_host();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/"#), r#"/"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_in_host_2() {
    let router = setup_marker_in_host();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/"#), r#"/"#.to_string(),Some(r#"test.com"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_in_host_3() {
    let router = setup_marker_in_host();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/"#), r#"/"#.to_string(),Some(r#"www.test.com"#.to_string()),Some(r#"https"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"https://www.test.io"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_in_querystring() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"matchany-rule","markers":[{"name":"marker","regex":"(?:.+?)"}],"rank":0,"source":{"path":"/a@marker"},"status_code":302,"target":"/b@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"querystring-rule","markers":[{"name":"marker","regex":"([\\p{Ll}])+?"}],"rank":0,"source":{"path":"/querystring/from","query":"slug=@marker"},"status_code":302,"target":"/querystring/target/some-target/@marker.html"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_marker_in_querystring_1() {
    let router = setup_marker_in_querystring();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/querystring/from?slug=coucou"#), r#"/querystring/from?slug=coucou"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/querystring/target/some-target/coucou.html"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_in_querystring_2() {
    let router = setup_marker_in_querystring();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/querystring/from?slug=2048"#), r#"/querystring/from?slug=2048"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_in_querystring_3() {
    let router = setup_marker_in_querystring();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/querystring/from"#), r#"/querystring/from"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_in_querystring_4() {
    let router = setup_marker_in_querystring();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/a?yolo=yala"#), r#"/a?yolo=yala"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/b?yolo=yala"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_transformation_camelize() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"camelize-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?","transformers":[{"options":null,"type":"camelize"}]}],"rank":0,"source":{"path":"/camelize/from/@marker"},"status_code":302,"target":"/camelize/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_transformation_camelize_1() {
    let router = setup_marker_transformation_camelize();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/helloPoney"#), r#"/camelize/from/helloPoney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPoney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_camelize_2() {
    let router = setup_marker_transformation_camelize();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/Hello-poney"#), r#"/camelize/from/Hello-poney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPoney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_camelize_3() {
    let router = setup_marker_transformation_camelize();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/HelloPoney"#), r#"/camelize/from/HelloPoney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPoney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_camelize_4() {
    let router = setup_marker_transformation_camelize();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/hello-pOney"#), r#"/camelize/from/hello-pOney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPOney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_transformation_dasherize() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"dasherize-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?","transformers":[{"options":null,"type":"dasherize"}]}],"rank":0,"source":{"path":"/dasherize/from/@marker"},"status_code":302,"target":"/dasherize/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_transformation_dasherize_1() {
    let router = setup_marker_transformation_dasherize();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/dasherize/from/HelloPoney"#), r#"/dasherize/from/HelloPoney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/dasherize/target/hello-poney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_dasherize_2() {
    let router = setup_marker_transformation_dasherize();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/dasherize/from/helloPoney"#), r#"/dasherize/from/helloPoney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/dasherize/target/hello-poney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_dasherize_3() {
    let router = setup_marker_transformation_dasherize();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/dasherize/from/Hello-Poney"#), r#"/dasherize/from/Hello-Poney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/dasherize/target/hello-poney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_transformation_lowercase() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"lowercase-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?","transformers":[{"options":null,"type":"lowercase"}]}],"rank":0,"source":{"path":"/lowercase/from/@marker"},"status_code":302,"target":"/lowercase/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_transformation_lowercase_1() {
    let router = setup_marker_transformation_lowercase();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/lowercase/from/HELLO-PONEY"#), r#"/lowercase/from/HELLO-PONEY"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/lowercase/target/hello-poney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_lowercase_2() {
    let router = setup_marker_transformation_lowercase();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/lowercase/from/HeLlO-PoNeY"#), r#"/lowercase/from/HeLlO-PoNeY"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/lowercase/target/hello-poney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_lowercase_3() {
    let router = setup_marker_transformation_lowercase();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/lowercase/from/hello-poney"#), r#"/lowercase/from/hello-poney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/lowercase/target/hello-poney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_transformation_replace() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"replace-rule","markers":[{"name":"marker","regex":"(cat|dog|fish)","transformers":[{"options":{"something":"cat","with":"tiger"},"type":"replace"}]}],"rank":0,"source":{"path":"/replace/from/@marker"},"status_code":302,"target":"/replace/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"replace-rule-2","markers":[{"name":"marker","regex":".*","transformers":[{"options":{"something":"disappear","with":""},"type":"replace"}]}],"rank":0,"source":{"path":"/replace-2/from/@marker"},"status_code":302,"target":"/@marker"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_marker_transformation_replace_1() {
    let router = setup_marker_transformation_replace();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/replace/from/poney"#), r#"/replace/from/poney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_transformation_replace_2() {
    let router = setup_marker_transformation_replace();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/replace/from/cat"#), r#"/replace/from/cat"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/replace/target/tiger"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_replace_3() {
    let router = setup_marker_transformation_replace();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/replace/from/dog"#), r#"/replace/from/dog"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/replace/target/dog"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_replace_4() {
    let router = setup_marker_transformation_replace();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/replace-2/from/disappear"#), r#"/replace-2/from/disappear"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_replace_5() {
    let router = setup_marker_transformation_replace();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/replace-2/from/something"#), r#"/replace-2/from/something"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/something"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_transformation_slice() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"slice-middle-rule","markers":[{"name":"marker","regex":"([\\p{Lu}\\p{Lt}])+?","transformers":[{"options":{"from":"5","to":"15"},"type":"slice"}]}],"rank":0,"source":{"path":"/slice-middle/from/@marker"},"status_code":302,"target":"/slice-middle/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"slice-rule","markers":[{"name":"marker","regex":"([\\p{Lu}\\p{Lt}])+?","transformers":[{"options":{"from":"0","to":"10"},"type":"slice"}]}],"rank":0,"source":{"path":"/slice/from/@marker"},"status_code":302,"target":"/slice/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"slice-rule-4-6","markers":[{"name":"marker","regex":".*","transformers":[{"options":{"from":"4","to":"6"},"type":"slice"}]}],"rank":0,"source":{"path":"/slice-rule-4-6/from/@marker"},"status_code":302,"target":"/@marker"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    router
}


#[test]
fn test_marker_transformation_slice_1() {
    let router = setup_marker_transformation_slice();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/slice/from/ABCDEFGHIJKLMNOPQRSTUVWXYZ"#), r#"/slice/from/ABCDEFGHIJKLMNOPQRSTUVWXYZ"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/slice/target/ABCDEFGHIJ"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_slice_2() {
    let router = setup_marker_transformation_slice();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/slice/from/ABCD"#), r#"/slice/from/ABCD"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/slice/target/ABCD"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_slice_3() {
    let router = setup_marker_transformation_slice();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/slice-middle/from/ABCDEFGHIJKLMNOPQRSTUVWXYZ"#), r#"/slice-middle/from/ABCDEFGHIJKLMNOPQRSTUVWXYZ"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/slice-middle/target/FGHIJKLMNO"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_slice_4() {
    let router = setup_marker_transformation_slice();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/slice-middle/from/ABCDEFGHIJ"#), r#"/slice-middle/from/ABCDEFGHIJ"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/slice-middle/target/FGHIJ"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_slice_5() {
    let router = setup_marker_transformation_slice();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/slice-middle/from/ABCD"#), r#"/slice-middle/from/ABCD"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/slice-middle/target/"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_slice_6() {
    let router = setup_marker_transformation_slice();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/slice-rule-4-6/from/anything"#), r#"/slice-rule-4-6/from/anything"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/hi"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_slice_7() {
    let router = setup_marker_transformation_slice();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/slice-rule-4-6/from/hello"#), r#"/slice-rule-4-6/from/hello"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/o"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_slice_8() {
    let router = setup_marker_transformation_slice();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/slice-rule-4-6/from/hey"#), r#"/slice-rule-4-6/from/hey"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_transformation_underscorize() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"underscorize-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-|_)+?","transformers":[{"options":null,"type":"underscorize"}]}],"rank":0,"source":{"path":"/underscorize/from/@marker"},"status_code":302,"target":"/underscorize/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_transformation_underscorize_1() {
    let router = setup_marker_transformation_underscorize();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/underscorize/from/hello_poney"#), r#"/underscorize/from/hello_poney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/underscorize/target/hello_poney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_underscorize_2() {
    let router = setup_marker_transformation_underscorize();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/underscorize/from/hello-poney"#), r#"/underscorize/from/hello-poney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/underscorize/target/hello_poney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_underscorize_3() {
    let router = setup_marker_transformation_underscorize();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/underscorize/from/HelloPoney"#), r#"/underscorize/from/HelloPoney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/underscorize/target/hello_poney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_underscorize_4() {
    let router = setup_marker_transformation_underscorize();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/underscorize/from/helloPoney"#), r#"/underscorize/from/helloPoney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/underscorize/target/hello_poney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_transformation_uppercase() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"uppercase-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?","transformers":[{"options":null,"type":"uppercase"}]}],"rank":0,"source":{"path":"/uppercase/from/@marker"},"status_code":302,"target":"/uppercase/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_transformation_uppercase_1() {
    let router = setup_marker_transformation_uppercase();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/uppercase/from/HELLO-PONEY"#), r#"/uppercase/from/HELLO-PONEY"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/uppercase/target/HELLO-PONEY"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_uppercase_2() {
    let router = setup_marker_transformation_uppercase();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/uppercase/from/HeLlO-PoNeY"#), r#"/uppercase/from/HeLlO-PoNeY"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/uppercase/target/HELLO-PONEY"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_transformation_uppercase_3() {
    let router = setup_marker_transformation_uppercase();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/uppercase/from/hello-poney"#), r#"/uppercase/from/hello-poney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/uppercase/target/HELLO-PONEY"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_type_anything() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"anything-rule","markers":[{"name":"marker","regex":".*"}],"rank":0,"source":{"path":"/anything/from/@marker"},"status_code":302,"target":"/anything/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_type_anything_1() {
    let router = setup_marker_type_anything();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/anything/from/f6883ff9-f163-43d7-8177-bfa24277fd20"#), r#"/anything/from/f6883ff9-f163-43d7-8177-bfa24277fd20"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/anything/target/f6883ff9-f163-43d7-8177-bfa24277fd20"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_anything_2() {
    let router = setup_marker_type_anything();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/anything/from/HELLO"#), r#"/anything/from/HELLO"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/anything/target/HELLO"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_anything_3() {
    let router = setup_marker_type_anything();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/anything/from/🤘"#), r#"/anything/from/🤘"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/anything/target/%F0%9F%A4%98"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_type_date() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"date-rule","markers":[{"name":"marker","regex":"([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])"}],"rank":0,"source":{"path":"/date/from/@marker"},"status_code":302,"target":"/date/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_type_date_1() {
    let router = setup_marker_type_date();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/date/from/2018-11-23"#), r#"/date/from/2018-11-23"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/date/target/2018-11-23"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_date_2() {
    let router = setup_marker_type_date();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/date/from/2018-23-11"#), r#"/date/from/2018-23-11"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_date_3() {
    let router = setup_marker_type_date();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/date/from/some-13-01"#), r#"/date/from/some-13-01"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_marker_type_datetime() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"datetime-rule","markers":[{"name":"marker","regex":"([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])T([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\\.[0-9]+)?(([Zz])|([\\+|\\-]([01][0-9]|2[0-3])(:?[03]0)?))"}],"rank":0,"source":{"path":"/datetime/from/@marker"},"status_code":302,"target":"/datetime/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"datetime-rule-with-transform","markers":[{"name":"marker","regex":"([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])T([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\\.[0-9]+)?(([Zz])|([\\+|\\-]([01][0-9]|2[0-3])(:?[03]0)?))","transformers":[{"options":{"from":"0","to":"10"},"type":"slice"}]}],"rank":0,"source":{"path":"/datetime-transform/from/@marker"},"status_code":302,"target":"/datetime-transform/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_marker_type_datetime_1() {
    let router = setup_marker_type_datetime();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/datetime/from/2018-07-15T14:59:12Z"#), r#"/datetime/from/2018-07-15T14:59:12Z"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/datetime/target/2018-07-15T14:59:12Z"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_datetime_2() {
    let router = setup_marker_type_datetime();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/datetime/from/2018-07-15T14:59:12+02:00"#), r#"/datetime/from/2018-07-15T14:59:12+02:00"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/datetime/target/2018-07-15T14:59:12+02:00"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_datetime_3() {
    let router = setup_marker_type_datetime();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/datetime/from/2018-07-15 14:59:12Z"#), r#"/datetime/from/2018-07-15 14:59:12Z"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_datetime_4() {
    let router = setup_marker_type_datetime();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/datetime-transform/from/2018-07-15T14:59:12Z"#), r#"/datetime-transform/from/2018-07-15T14:59:12Z"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/datetime-transform/target/2018-07-15"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_marker_type_enum() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"enum-rule","markers":[{"name":"marker","regex":"(cat|dog|fish)"}],"rank":0,"source":{"path":"/enum/from/@marker"},"status_code":302,"target":"/enum/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_type_enum_1() {
    let router = setup_marker_type_enum();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/enum/from/cat"#), r#"/enum/from/cat"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/enum/target/cat"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_enum_2() {
    let router = setup_marker_type_enum();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/enum/from/cats-eyes"#), r#"/enum/from/cats-eyes"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_enum_3() {
    let router = setup_marker_type_enum();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/enum/from/dog"#), r#"/enum/from/dog"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/enum/target/dog"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_enum_4() {
    let router = setup_marker_type_enum();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/enum/from/dogville"#), r#"/enum/from/dogville"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_marker_type_integer() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"integer-max-rule","markers":[{"name":"marker","regex":"([0-9]|[1-3][0-9]|4[0-2])"}],"rank":0,"source":{"path":"/integer-max/from/@marker"},"status_code":302,"target":"/integer-max/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"integer-min-max-rule","markers":[{"name":"marker","regex":"(4[2-9]|[5-9][0-9]|[1-9][0-9]{2}|1[0-2][0-9]{2}|13[0-2][0-9]|133[0-7])"}],"rank":0,"source":{"path":"/integer-min-max/from/@marker"},"status_code":302,"target":"/integer-min-max/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"integer-min-rule","markers":[{"name":"marker","regex":"[1-3][0-9]{2,}|4([1-1][0-9]{1,}|[2-9][0-9]*)|[5-9][0-9]{1,}"}],"rank":0,"source":{"path":"/integer-min/from/@marker"},"status_code":302,"target":"/integer-min/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    let route_4: Rule = serde_json::from_str(r#"{"id":"integer-rule","markers":[{"name":"marker","regex":"[0-9]+"}],"rank":0,"source":{"path":"/integer/from/@marker"},"status_code":302,"target":"/integer/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_4.into_route(&router.config));

    router
}


#[test]
fn test_marker_type_integer_1() {
    let router = setup_marker_type_integer();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/integer/from/2778"#), r#"/integer/from/2778"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/integer/target/2778"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_integer_2() {
    let router = setup_marker_type_integer();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/integer/from/42l33t"#), r#"/integer/from/42l33t"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_integer_3() {
    let router = setup_marker_type_integer();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/integer/from/42-l33t"#), r#"/integer/from/42-l33t"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_integer_4() {
    let router = setup_marker_type_integer();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/integer-min/from/112"#), r#"/integer-min/from/112"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/integer-min/target/112"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_integer_5() {
    let router = setup_marker_type_integer();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/integer-min/from/11"#), r#"/integer-min/from/11"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_integer_6() {
    let router = setup_marker_type_integer();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/integer-max/from/11"#), r#"/integer-max/from/11"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/integer-max/target/11"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_integer_7() {
    let router = setup_marker_type_integer();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/integer-max/from/112"#), r#"/integer-max/from/112"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_integer_8() {
    let router = setup_marker_type_integer();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/integer-min-max/from/806"#), r#"/integer-min-max/from/806"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/integer-min-max/target/806"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_integer_9() {
    let router = setup_marker_type_integer();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/integer-min-max/from/33"#), r#"/integer-min-max/from/33"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_integer_10() {
    let router = setup_marker_type_integer();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/integer-min-max/from/2048"#), r#"/integer-min-max/from/2048"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_marker_type_string() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"string-allowLowercaseAlphabet-specificCharacters-starting-containing-rule","markers":[{"name":"marker","regex":"JOHN\\-SNOW(([\\p{Ll}]|\\-)*?L33T([\\p{Ll}]|\\-)*?)+?"}],"rank":0,"source":{"path":"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/@marker"},"status_code":302,"target":"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"string-allowPercentEncodedChars-rule","markers":[{"name":"marker","regex":"(%[0-9A-Z]{2})+?"}],"rank":0,"source":{"path":"/string-allowPercentEncodedChars/from/@marker"},"status_code":302,"target":"/string-allowPercentEncodedChars/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"string-containing-rule","markers":[{"name":"marker","regex":"(L33T)+?"}],"rank":0,"source":{"path":"/string-containing/from/@marker"},"status_code":302,"target":"/string-containing/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    let route_4: Rule = serde_json::from_str(r#"{"id":"string-ending-rule","markers":[{"name":"marker","regex":"([\\p{Ll}]|\\-)+?JOHN\\-SNOW"}],"rank":0,"source":{"path":"/string-ending/from/@marker"},"status_code":302,"target":"/string-ending/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_4.into_route(&router.config));

    let route_5: Rule = serde_json::from_str(r#"{"id":"string-lowercase-digits-allowPercentEncodedChars-rule","markers":[{"name":"marker","regex":"([\\p{Ll}0-9]|%[0-9A-Z]{2})+?"}],"rank":0,"source":{"path":"/string-lowercase-digits-allowPercentEncodedChars/from/@marker"},"status_code":302,"target":"/string-lowercase-digits-allowPercentEncodedChars/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_5.into_route(&router.config));

    let route_6: Rule = serde_json::from_str(r#"{"id":"string-lowercase-rule","markers":[{"name":"marker","regex":"([\\p{Ll}])+?"}],"rank":0,"source":{"path":"/string-lowercase/from/@marker"},"status_code":302,"target":"/string-lowercase/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_6.into_route(&router.config));

    let route_7: Rule = serde_json::from_str(r#"{"id":"string-lowercase-specificCharacters-emoji-rule","markers":[{"name":"marker","regex":"([\\p{Ll}]|\\-|🤘)+?"}],"rank":0,"source":{"path":"/string-lowercase-specificCharacters-emoji/from/@marker"},"status_code":302,"target":"/string-lowercase-specificCharacters-emoji/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_7.into_route(&router.config));

    let route_8: Rule = serde_json::from_str(r#"{"id":"string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}0-9]|\\-|\\.|\\(|\\)|%[0-9A-Z]{2})+?"}],"rank":0,"source":{"path":"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/from/@marker"},"status_code":302,"target":"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_8.into_route(&router.config));

    let route_9: Rule = serde_json::from_str(r#"{"id":"string-lowercase-uppercase-digits-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}0-9])+?"}],"rank":0,"source":{"path":"/string-lowercase-uppercase-digits/from/@marker"},"status_code":302,"target":"/string-lowercase-uppercase-digits/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_9.into_route(&router.config));

    let route_10: Rule = serde_json::from_str(r#"{"id":"string-rule","markers":[{"name":"marker","regex":""}],"rank":0,"source":{"path":"/string/from/@marker"},"status_code":302,"target":"/string/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_10.into_route(&router.config));

    let route_11: Rule = serde_json::from_str(r#"{"id":"string-specificCharacters-other-rule","markers":[{"name":"marker","regex":"(a|\\-|z)+?"}],"rank":0,"source":{"path":"/string-specificCharacters-other/from/@marker"},"status_code":302,"target":"/string-specificCharacters-other/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_11.into_route(&router.config));

    let route_12: Rule = serde_json::from_str(r#"{"id":"string-specificCharacters-rule","markers":[{"name":"marker","regex":"(\\.|\\-|\\+|_|/|=)+?"}],"rank":0,"source":{"path":"/string-specificCharacters/from/@marker"},"status_code":302,"target":"/string-specificCharacters/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_12.into_route(&router.config));

    let route_13: Rule = serde_json::from_str(r#"{"id":"string-starting-rule","markers":[{"name":"marker","regex":"JOHN\\-SNOW([\\p{Ll}]|\\-)+?"}],"rank":0,"source":{"path":"/string-starting/from/@marker"},"status_code":302,"target":"/string-starting/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_13.into_route(&router.config));

    let route_14: Rule = serde_json::from_str(r#"{"id":"string-starting-shit-rule","markers":[{"name":"marker","regex":"\\(\\[A\\-Z\\]\\)\\+([\\p{Ll}]|\\-)+?"}],"rank":0,"source":{"path":"/string-starting-shit/from/@marker"},"status_code":302,"target":"/string-starting-shit/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_14.into_route(&router.config));

    let route_15: Rule = serde_json::from_str(r#"{"id":"string-uppercase-rule","markers":[{"name":"marker","regex":"([\\p{Lu}\\p{Lt}])+?"}],"rank":0,"source":{"path":"/string-uppercase/from/@marker"},"status_code":302,"target":"/string-uppercase/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_15.into_route(&router.config));

    router
}


#[test]
fn test_marker_type_string_1() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string/from/coucou"#), r#"/string/from/coucou"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_2() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase/from/coucou"#), r#"/string-lowercase/from/coucou"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase/target/coucou"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_3() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase/from/COUCOU"#), r#"/string-lowercase/from/COUCOU"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_4() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase/from/some-string"#), r#"/string-lowercase/from/some-string"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_5() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase/from/l33t"#), r#"/string-lowercase/from/l33t"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_6() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-uppercase/from/COUCOU"#), r#"/string-uppercase/from/COUCOU"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-uppercase/target/COUCOU"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_7() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-uppercase/from/coucou"#), r#"/string-uppercase/from/coucou"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_8() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-uppercase/from/SOME-STRING"#), r#"/string-uppercase/from/SOME-STRING"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_9() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-uppercase/from/L33T"#), r#"/string-uppercase/from/L33T"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_10() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase-uppercase-digits/from/coucou"#), r#"/string-lowercase-uppercase-digits/from/coucou"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits/target/coucou"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_11() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase-uppercase-digits/from/COUCOU"#), r#"/string-lowercase-uppercase-digits/from/COUCOU"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits/target/COUCOU"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_12() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase-uppercase-digits/from/SOME-STRING"#), r#"/string-lowercase-uppercase-digits/from/SOME-STRING"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_13() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase-uppercase-digits/from/l33t"#), r#"/string-lowercase-uppercase-digits/from/l33t"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits/target/l33t"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_14() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase-uppercase-digits/from/L33T"#), r#"/string-lowercase-uppercase-digits/from/L33T"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits/target/L33T"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_15() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-specificCharacters/from/-"#), r#"/string-specificCharacters/from/-"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-specificCharacters/target/-"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_16() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-specificCharacters/from/-_.+_-/._-_."#), r#"/string-specificCharacters/from/-_.+_-/._-_."#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-specificCharacters/target/-_.+_-/._-_."#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_17() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-specificCharacters-other/from/z-a-z-a-zz"#), r#"/string-specificCharacters-other/from/z-a-z-a-zz"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-specificCharacters-other/target/z-a-z-a-zz"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_18() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-specificCharacters-other/from/azerty"#), r#"/string-specificCharacters-other/from/azerty"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_19() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase-specificCharacters-emoji/from/you-rock-dude-🤘"#), r#"/string-lowercase-specificCharacters-emoji/from/you-rock-dude-🤘"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-specificCharacters-emoji/target/you-rock-dude-%F0%9F%A4%98"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_20() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-starting/from/JOHN-SNOW-knows-nothing"#), r#"/string-starting/from/JOHN-SNOW-knows-nothing"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-starting/target/JOHN-SNOW-knows-nothing"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_21() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-starting/from/you-know-nothing-JOHN-SNOW"#), r#"/string-starting/from/you-know-nothing-JOHN-SNOW"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_22() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-starting-shit/from/COUCOU-you-know-nothing"#), r#"/string-starting-shit/from/COUCOU-you-know-nothing"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_23() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-starting-shit/from/([A-Z])+-knows-nothing"#), r#"/string-starting-shit/from/([A-Z])+-knows-nothing"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-starting-shit/target/([A-Z])+-knows-nothing"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_24() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-ending/from/JOHN-SNOW-knows-nothing"#), r#"/string-ending/from/JOHN-SNOW-knows-nothing"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_25() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-ending/from/you-know-nothing-JOHN-SNOW"#), r#"/string-ending/from/you-know-nothing-JOHN-SNOW"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-ending/target/you-know-nothing-JOHN-SNOW"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_26() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-ending/from/you-know-nothing-JOHN-SNOWR"#), r#"/string-ending/from/you-know-nothing-JOHN-SNOWR"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_27() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-allowPercentEncodedChars/from/%2B%3A%26"#), r#"/string-allowPercentEncodedChars/from/%2B%3A%26"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowPercentEncodedChars/target/%2B%3A%26"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_28() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-allowPercentEncodedChars/from/%3A"#), r#"/string-allowPercentEncodedChars/from/%3A"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowPercentEncodedChars/target/%3A"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_29() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-allowPercentEncodedChars/from/%2B"#), r#"/string-allowPercentEncodedChars/from/%2B"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowPercentEncodedChars/target/%2B"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_30() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-allowPercentEncodedChars/from/%26"#), r#"/string-allowPercentEncodedChars/from/%26"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowPercentEncodedChars/target/%26"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_31() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-allowPercentEncodedChars/from/0%2B0%3Dtoto"#), r#"/string-allowPercentEncodedChars/from/0%2B0%3Dtoto"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_32() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-allowPercentEncodedChars/from/+:&"#), r#"/string-allowPercentEncodedChars/from/+:&"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_33() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase-digits-allowPercentEncodedChars/from/0%2B0%3Dtoto"#), r#"/string-lowercase-digits-allowPercentEncodedChars/from/0%2B0%3Dtoto"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-digits-allowPercentEncodedChars/target/0%2B0%3Dtoto"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_34() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase-digits-allowPercentEncodedChars/from/0+0=toto"#), r#"/string-lowercase-digits-allowPercentEncodedChars/from/0+0=toto"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_35() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/from/Medios-de-Comunicaci%C3%B3n-y-Creatividad"#), r#"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/from/Medios-de-Comunicaci%C3%B3n-y-Creatividad"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/target/Medios-de-Comunicaci%C3%B3n-y-Creatividad"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_36() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/from/Medios-de-Comunicación-y-Creatividad"#), r#"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/from/Medios-de-Comunicación-y-Creatividad"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-lowercase-uppercase-digits-allowPercentEncodedChars-specificCharacters/target/Medios-de-Comunicaci%C3%B3n-y-Creatividad"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_37() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-containing/from/L33T"#), r#"/string-containing/from/L33T"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-containing/target/L33T"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_38() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-containing/from/L33TL33T"#), r#"/string-containing/from/L33TL33T"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-containing/target/L33TL33T"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_39() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-containing/from/42-L33T-42"#), r#"/string-containing/from/42-L33T-42"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_40() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOW-L33T-knows-nothing"#), r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOW-L33T-knows-nothing"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/target/JOHN-SNOW-L33T-knows-nothing"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_41() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOWL33T"#), r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOWL33T"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/target/JOHN-SNOWL33T"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_string_42() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/L33T-JOHN-SNOW-knows-nothing"#), r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/L33T-JOHN-SNOW-knows-nothing"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_43() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOW-l33t"#), r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOW-l33t"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_string_44() {
    let router = setup_marker_type_string();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOW-L3a3t"#), r#"/string-allowLowercaseAlphabet-specificCharacters-starting-containing/from/JOHN-SNOW-L3a3t"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_marker_type_uuid() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"uuid-rule","markers":[{"name":"marker","regex":"[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}"}],"rank":0,"source":{"path":"/uuid/from/@marker"},"status_code":302,"target":"/uuid/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marker_type_uuid_1() {
    let router = setup_marker_type_uuid();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/uuid/from/f6883ff9-f163-43d7-8177-bfa24277fd20"#), r#"/uuid/from/f6883ff9-f163-43d7-8177-bfa24277fd20"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/uuid/target/f6883ff9-f163-43d7-8177-bfa24277fd20"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marker_type_uuid_2() {
    let router = setup_marker_type_uuid();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/uuid/from/HELLO"#), r#"/uuid/from/HELLO"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marker_type_uuid_3() {
    let router = setup_marker_type_uuid();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/uuid/from/f688-3ff9-f16343d78177bfa2-4277-fd20"#), r#"/uuid/from/f688-3ff9-f16343d78177bfa2-4277-fd20"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_marketing_parameters() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["param1","param2"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule","rank":0,"source":{"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marketing_parameters_1() {
    let router = setup_marketing_parameters();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marketing_parameters_2() {
    let router = setup_marketing_parameters();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param1=value1"#), r#"/foo?param1=value1"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar?param1=value1"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marketing_parameters_3() {
    let router = setup_marketing_parameters();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param1=value1&param2=value2"#), r#"/foo?param1=value1&param2=value2"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar?param1=value1&param2=value2"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marketing_parameters_4() {
    let router = setup_marketing_parameters();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param2=value1&param1=value2"#), r#"/foo?param2=value1&param1=value2"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar?param1=value2&param2=value1"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marketing_parameters_5() {
    let router = setup_marketing_parameters();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param3=value3"#), r#"/foo?param3=value3"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marketing_parameters_6() {
    let router = setup_marketing_parameters();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param1=value1&param3=value3"#), r#"/foo?param1=value1&param3=value3"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_marketing_parameters_notarget() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["param1","param2"],"pass_marketing_query_params_to_target":false}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule","rank":0,"source":{"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marketing_parameters_notarget_1() {
    let router = setup_marketing_parameters_notarget();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marketing_parameters_notarget_2() {
    let router = setup_marketing_parameters_notarget();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param1=value1"#), r#"/foo?param1=value1"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marketing_parameters_notarget_3() {
    let router = setup_marketing_parameters_notarget();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param1=value1&param2=value2"#), r#"/foo?param1=value1&param2=value2"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marketing_parameters_notarget_4() {
    let router = setup_marketing_parameters_notarget();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param2=value1&param1=value2"#), r#"/foo?param2=value1&param1=value2"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marketing_parameters_notarget_5() {
    let router = setup_marketing_parameters_notarget();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param3=value3"#), r#"/foo?param3=value3"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_marketing_parameters_notarget_6() {
    let router = setup_marketing_parameters_notarget();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param1=value1&param3=value3"#), r#"/foo?param1=value1&param3=value3"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_marketing_parameters_with_catch_all() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_campaing","utm_content","utm_medium","utm_source","utm_term"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule","markers":[{"name":"params","regex":"(\\?.*)?$"}],"rank":0,"source":{"path":"/us/en/story/276298-christmas-2020/@params"},"status_code":301,"target":"/us/en/story/275996-women-gifts/@params"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_marketing_parameters_with_catch_all_1() {
    let router = setup_marketing_parameters_with_catch_all();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/us/en/story/276298-christmas-2020/"#), r#"/us/en/story/276298-christmas-2020/"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/us/en/story/275996-women-gifts/"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marketing_parameters_with_catch_all_2() {
    let router = setup_marketing_parameters_with_catch_all();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/us/en/story/276298-christmas-2020/?utm_test=123"#), r#"/us/en/story/276298-christmas-2020/?utm_test=123"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/us/en/story/275996-women-gifts/?utm_test=123"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marketing_parameters_with_catch_all_3() {
    let router = setup_marketing_parameters_with_catch_all();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/us/en/story/276298-christmas-2020/?utm_randomstring=123"#), r#"/us/en/story/276298-christmas-2020/?utm_randomstring=123"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/us/en/story/275996-women-gifts/?utm_randomstring=123"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_marketing_parameters_with_catch_all_4() {
    let router = setup_marketing_parameters_with_catch_all();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/us/en/story/276298-christmas-2020/?utm_source=123"#), r#"/us/en/story/276298-christmas-2020/?utm_source=123"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/us/en/story/275996-women-gifts/?utm_source=123"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_no_marketing_parameterst() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":false,"ignore_path_and_query_case":false,"marketing_query_params":["param1","param2"],"pass_marketing_query_params_to_target":false}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule","rank":0,"source":{"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_no_marketing_parameterst_1() {
    let router = setup_no_marketing_parameterst();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_no_marketing_parameterst_2() {
    let router = setup_no_marketing_parameterst();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param1=value1"#), r#"/foo?param1=value1"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_no_marketing_parameterst_3() {
    let router = setup_no_marketing_parameterst();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param1=value1&param2=value2"#), r#"/foo?param1=value1&param2=value2"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_no_marketing_parameterst_4() {
    let router = setup_no_marketing_parameterst();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param2=value1&param1=value2"#), r#"/foo?param2=value1&param1=value2"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_no_marketing_parameterst_5() {
    let router = setup_no_marketing_parameterst();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param3=value3"#), r#"/foo?param3=value3"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_no_marketing_parameterst_6() {
    let router = setup_no_marketing_parameterst();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo?param1=value1&param3=value3"#), r#"/foo?param1=value1&param3=value3"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_rule_any_host_match() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":true,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":false,"ignore_path_and_query_case":false,"marketing_query_params":[],"pass_marketing_query_params_to_target":false}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-with-host","rank":20,"source":{"host":"example.com","path":"/foo"},"status_code":302,"target":"/bar-example"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-without-host","rank":0,"source":{"path":"/foo"},"status_code":302,"target":"/bar-no-example"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_rule_any_host_match_1() {
    let router = setup_rule_any_host_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar-no-example"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_any_host_match_2() {
    let router = setup_rule_any_host_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),Some(r#"example.com"#.to_string()),None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar-no-example"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_any_host_no_match() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":false,"ignore_path_and_query_case":false,"marketing_query_params":[],"pass_marketing_query_params_to_target":false}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-with-host","rank":20,"source":{"host":"example.com","path":"/foo"},"status_code":302,"target":"/bar-example"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-without-host","rank":0,"source":{"path":"/foo"},"status_code":302,"target":"/bar-no-example"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_rule_any_host_no_match_1() {
    let router = setup_rule_any_host_no_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar-no-example"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_any_host_no_match_2() {
    let router = setup_rule_any_host_no_match();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),Some(r#"example.com"#.to_string()),None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar-example"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_header_regex() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-multiple-headers","markers":[{"name":"marker","regex":"^(ES|FR|IT)$"}],"rank":0,"source":{"headers":[{"name":"X-GeoIP","type":"match_regex","value":"@marker"}],"path":"/test"},"status_code":302,"target":"/es"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_header_regex_1() {
    let router = setup_rule_header_regex();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/test"#), r#"/test"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_header_regex_2() {
    let router = setup_rule_header_regex();
    let default_config = RouterConfig::default();
    let mut request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/test"#), r#"/test"#.to_string(),None,None,None,None,None);
    request.add_header(r#"X-GeoIP"#.to_string(), r#"EN"#.to_string(), false);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_header_regex_3() {
    let router = setup_rule_header_regex();
    let default_config = RouterConfig::default();
    let mut request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/test"#), r#"/test"#.to_string(),None,None,None,None,None);
    request.add_header(r#"X-GeoIP"#.to_string(), r#"FR"#.to_string(), false);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/es"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_ip_trigger() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-in-range","rank":0,"source":{"ips":[{"in_range":"192.168.0.0/24"},{"in_range":"172.12.0.0/24"}],"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-not-in-range","rank":0,"source":{"ips":[{"not_in_range":"10.0.0.0/24"}],"path":"/foo2"},"status_code":302,"target":"/bar2"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_rule_ip_trigger_1() {
    let router = setup_rule_ip_trigger();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.1.12"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_ip_trigger_2() {
    let router = setup_rule_ip_trigger();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.12"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_ip_trigger_3() {
    let router = setup_rule_ip_trigger();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"172.12.0.1"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_ip_trigger_4() {
    let router = setup_rule_ip_trigger();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo2"#), r#"/foo2"#.to_string(),None,None,None,r#"192.168.1.12"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_ip_trigger_5() {
    let router = setup_rule_ip_trigger();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo2"#), r#"/foo2"#.to_string(),None,None,None,r#"10.0.0.1"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}


fn setup_rule_ip_trigger_equals() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-ip-trigger-equals","rank":0,"source":{"ips":[{"in_range":"192.168.0.1"}],"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_ip_trigger_equals_1() {
    let router = setup_rule_ip_trigger_equals();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.2"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_ip_trigger_equals_2() {
    let router = setup_rule_ip_trigger_equals();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.1"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_ip_trigger_greater_than() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-ip-trigger-equals","rank":0,"source":{"ips":[{"in_range":"192.169.0.0/16"},{"in_range":"192.170.0.0/15"},{"in_range":"192.172.0.0/14"},{"in_range":"192.176.0.0/12"},{"in_range":"192.192.0.0/10"},{"in_range":"193.0.0.0/8"},{"in_range":"194.0.0.0/7"},{"in_range":"196.0.0.0/6"},{"in_range":"200.0.0.0/5"},{"in_range":"208.0.0.0/4"},{"in_range":"224.0.0.0/3"}],"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_ip_trigger_greater_than_1() {
    let router = setup_rule_ip_trigger_greater_than();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.2"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_ip_trigger_greater_than_2() {
    let router = setup_rule_ip_trigger_greater_than();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.180.0.0"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_ip_trigger_greater_than_3() {
    let router = setup_rule_ip_trigger_greater_than();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"200.168.0.0"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_ip_trigger_greater_than_or_equals() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-ip-trigger-equals","rank":0,"source":{"ips":[{"in_range":"192.168.255.255/32"},{"in_range":"192.169.0.0/16"},{"in_range":"192.170.0.0/15"},{"in_range":"192.172.0.0/14"},{"in_range":"192.176.0.0/12"},{"in_range":"192.192.0.0/10"},{"in_range":"193.0.0.0/8"},{"in_range":"194.0.0.0/7"},{"in_range":"196.0.0.0/6"},{"in_range":"200.0.0.0/5"},{"in_range":"208.0.0.0/4"},{"in_range":"224.0.0.0/3"}],"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_ip_trigger_greater_than_or_equals_1() {
    let router = setup_rule_ip_trigger_greater_than_or_equals();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.2"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_ip_trigger_greater_than_or_equals_2() {
    let router = setup_rule_ip_trigger_greater_than_or_equals();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.180.0.0"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_ip_trigger_greater_than_or_equals_3() {
    let router = setup_rule_ip_trigger_greater_than_or_equals();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"200.168.0.0"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_ip_trigger_in_range() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-ip-trigger-equals","rank":0,"source":{"ips":[{"in_range":"192.168.0.0/24"}],"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_ip_trigger_in_range_1() {
    let router = setup_rule_ip_trigger_in_range();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.1.2"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_ip_trigger_in_range_2() {
    let router = setup_rule_ip_trigger_in_range();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.1"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_ip_trigger_less_than() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-ip-trigger-equals","rank":0,"source":{"ips":[{"in_range":"0.0.0.0/1"},{"in_range":"128.0.0.0/2"},{"in_range":"192.0.0.0/9"},{"in_range":"192.128.0.0/11"},{"in_range":"192.160.0.0/13"},{"in_range":"192.168.0.0/32"}],"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_ip_trigger_less_than_1() {
    let router = setup_rule_ip_trigger_less_than();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.1"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_ip_trigger_less_than_2() {
    let router = setup_rule_ip_trigger_less_than();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.0"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_ip_trigger_less_than_3() {
    let router = setup_rule_ip_trigger_less_than();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"10.168.0.0"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_ip_trigger_less_than_or_equals() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-ip-trigger-equals","rank":0,"source":{"ips":[{"in_range":"0.0.0.0/1"},{"in_range":"128.0.0.0/2"},{"in_range":"192.0.0.0/9"},{"in_range":"192.128.0.0/11"},{"in_range":"192.160.0.0/13"},{"in_range":"192.168.0.0/31"}],"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_ip_trigger_less_than_or_equals_1() {
    let router = setup_rule_ip_trigger_less_than_or_equals();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.2"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_ip_trigger_less_than_or_equals_2() {
    let router = setup_rule_ip_trigger_less_than_or_equals();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.1"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_ip_trigger_less_than_or_equals_3() {
    let router = setup_rule_ip_trigger_less_than_or_equals();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"10.168.0.0"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_ip_trigger_not_equals() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-ip-trigger-equals","rank":0,"source":{"ips":[{"not_in_range":"192.168.0.1"}],"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_ip_trigger_not_equals_1() {
    let router = setup_rule_ip_trigger_not_equals();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.1"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_ip_trigger_not_equals_2() {
    let router = setup_rule_ip_trigger_not_equals();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.2"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_ip_trigger_not_in_range() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-ip-trigger-equals","rank":0,"source":{"ips":[{"not_in_range":"192.168.0.0/24"}],"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_ip_trigger_not_in_range_1() {
    let router = setup_rule_ip_trigger_not_in_range();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.0.2"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_ip_trigger_not_in_range_2() {
    let router = setup_rule_ip_trigger_not_in_range();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,r#"192.168.1.1"#.to_string().parse().ok(),None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_multiple_headers() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-multiple-headers","rank":0,"source":{"headers":[{"name":"X-Foo","type":"is_defined","value":null},{"name":"X-Bar","type":"is_defined","value":null}],"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_multiple_headers_1() {
    let router = setup_rule_multiple_headers();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_multiple_headers_2() {
    let router = setup_rule_multiple_headers();
    let default_config = RouterConfig::default();
    let mut request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    request.add_header(r#"X-Foo"#.to_string(), r#"foo"#.to_string(), false);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_multiple_headers_3() {
    let router = setup_rule_multiple_headers();
    let default_config = RouterConfig::default();
    let mut request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    request.add_header(r#"X-Bar"#.to_string(), r#"bar"#.to_string(), false);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_multiple_headers_4() {
    let router = setup_rule_multiple_headers();
    let default_config = RouterConfig::default();
    let mut request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    request.add_header(r#"X-Foo"#.to_string(), r#"foo"#.to_string(), false);
    request.add_header(r#"X-Bar"#.to_string(), r#"bar"#.to_string(), false);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_query_with_pipe() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"0_host-path-query-pipe-urlencoded","rank":0,"source":{"host":"example.org","path":"/query-pipe","query":"foo=bar%7Cbaz"},"status_code":301,"target":"/target-urlencoded"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"1_host-path-query-pipe","rank":0,"source":{"host":"example.org","path":"/query-pipe","query":"foo=bar|baz"},"status_code":301,"target":"/target"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_rule_query_with_pipe_1() {
    let router = setup_rule_query_with_pipe();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/query-pipe?foo=bar|baz"#), r#"/query-pipe?foo=bar|baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target-urlencoded"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_query_with_pipe_2() {
    let router = setup_rule_query_with_pipe();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/query-pipe?foo=bar%7Cbaz"#), r#"/query-pipe?foo=bar%7Cbaz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target-urlencoded"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_query_with_pipe_2() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"host-path-query-pipe","rank":0,"source":{"host":"example.org","path":"/query-pipe","query":"foo=bar|baz"},"status_code":301,"target":"/target"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_query_with_pipe_2_1() {
    let router = setup_rule_query_with_pipe_2();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/query-pipe?foo=bar|baz"#), r#"/query-pipe?foo=bar|baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_query_with_pipe_2_2() {
    let router = setup_rule_query_with_pipe_2();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/query-pipe?foo=bar%7Cbaz"#), r#"/query-pipe?foo=bar%7Cbaz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_query_with_plus() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"host-path-query-double-quotes","rank":0,"source":{"host":"example.org","path":"/query-plus","query":"foo=bar+baz"},"status_code":301,"target":"/target"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_query_with_plus_1() {
    let router = setup_rule_query_with_plus();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/query-plus?foo=bar%2Bbaz"#), r#"/query-plus?foo=bar%2Bbaz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_query_with_plus_2() {
    let router = setup_rule_query_with_plus();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/query-plus?foo=bar+baz"#), r#"/query-plus?foo=bar+baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_query_with_plus_3() {
    let router = setup_rule_query_with_plus();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/query-plus?foo=bar baz"#), r#"/query-plus?foo=bar baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_query_with_plus_4() {
    let router = setup_rule_query_with_plus();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/query-plus?foo=bar%20baz"#), r#"/query-plus?foo=bar%20baz"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_query_with_plus_2() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"host-path-query-double-quotes","markers":[{"name":"marker","regex":".+?bar.+?"}],"rank":0,"source":{"path":"/query-plus","query":"@marker"},"status_code":301,"target":"/target?@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_query_with_plus_2_1() {
    let router = setup_rule_query_with_plus_2();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/query-plus?foo=bar%2Bbaz"#), r#"/query-plus?foo=bar%2Bbaz"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target?foo=bar%2Bbaz"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_querystring() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"host-path-query-double-quotes","rank":0,"source":{"host":"example.org","path":"/host-path-query","query":"foo&bar=yolo"},"status_code":301,"target":"/target"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_querystring_1() {
    let router = setup_rule_querystring();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/host-path-query?foo&bar=yolo"#), r#"/host-path-query?foo&bar=yolo"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_querystring_2() {
    let router = setup_rule_querystring();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/host-path-query?foo=&bar=yolo"#), r#"/host-path-query?foo=&bar=yolo"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_sampling() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-sampled-0","rank":0,"source":{"path":"/foo","sampling":50},"status_code":302,"target":"/foo2"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-sampled-100","rank":0,"source":{"path":"/bar","sampling":50},"status_code":302,"target":"/bar2"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_rule_sampling_1() {
    let router = setup_rule_sampling();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,Some(false));
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_sampling_2() {
    let router = setup_rule_sampling();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/bar"#), r#"/bar"#.to_string(),None,None,None,None,Some(true));
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_skipped_query_parameters() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-1","rank":0,"source":{"path":"/source"},"status_code":301,"target":"/target"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-2","rank":0,"source":{"path":"/source","query":"toto=tata"},"status_code":301,"target":"/target?tutu=titi"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_rule_skipped_query_parameters_1() {
    let router = setup_rule_skipped_query_parameters();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source"#), r#"/source"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_skipped_query_parameters_2() {
    let router = setup_rule_skipped_query_parameters();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source?utm_source=test"#), r#"/source?utm_source=test"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target?utm_source=test"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_skipped_query_parameters_3() {
    let router = setup_rule_skipped_query_parameters();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source?toto=tata"#), r#"/source?toto=tata"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target?tutu=titi"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_skipped_query_parameters_4() {
    let router = setup_rule_skipped_query_parameters();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/source?toto=tata&utm_source=test&utm_content=test"#), r#"/source?toto=tata&utm_source=test&utm_content=test"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target?tutu=titi&utm_content=test&utm_source=test"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_with_header() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-header-marker","markers":[{"name":"marker","regex":"(?:f.+?)"}],"rank":0,"source":{"headers":[{"name":"X-Test-Marker","type":"match_regex","value":"@marker"}],"path":"/test"},"status_code":302,"target":"/baz/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-header-not-existing","rank":0,"source":{"headers":[{"name":"X-Test","type":"is_not_defined","value":null},{"name":"X-Test-Marker","type":"is_not_defined","value":null}],"path":"/test"},"status_code":302,"target":"/bor"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"rule-header-static","rank":0,"source":{"headers":[{"name":"X-Test","type":"contains","value":"foo"}],"path":"/test"},"status_code":302,"target":"/baz"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    router
}


#[test]
fn test_rule_with_header_1() {
    let router = setup_rule_with_header();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/test"#), r#"/test"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bor"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_header_2() {
    let router = setup_rule_with_header();
    let default_config = RouterConfig::default();
    let mut request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/test"#), r#"/test"#.to_string(),None,None,None,None,None);
    request.add_header(r#"X-Test"#.to_string(), r#"foo"#.to_string(), false);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/baz"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_header_3() {
    let router = setup_rule_with_header();
    let default_config = RouterConfig::default();
    let mut request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/test"#), r#"/test"#.to_string(),None,None,None,None,None);
    request.add_header(r#"X-Test-Marker"#.to_string(), r#"foo"#.to_string(), false);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/baz/foo"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_header_4() {
    let router = setup_rule_with_header();
    let default_config = RouterConfig::default();
    let mut request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/test"#), r#"/test"#.to_string(),None,None,None,None,None);
    request.add_header(r#"X-Test-Marker"#.to_string(), r#"unknown"#.to_string(), false);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_with_header_5() {
    let router = setup_rule_with_header();
    let default_config = RouterConfig::default();
    let mut request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/test"#), r#"/test"#.to_string(),None,None,None,None,None);
    request.add_header(r#"X-Test-Marker"#.to_string(), r#"unknown"#.to_string(), false);
    request.add_header(r#"X-Test-Marker"#.to_string(), r#"foofoo"#.to_string(), false);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/baz/foofoo"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_with_method() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-method-post","rank":0,"source":{"methods":["POST"],"path":"/foo"},"status_code":302,"target":"/baz"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-multiple-methods","rank":0,"source":{"methods":["PUT","POST"],"path":"/bar"},"status_code":302,"target":"/bor"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_rule_with_method_1() {
    let router = setup_rule_with_method();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,Some(r#"GET"#.to_string()),None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_with_method_2() {
    let router = setup_rule_with_method();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/bar"#), r#"/bar"#.to_string(),None,None,Some(r#"GET"#.to_string()),None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), false);
    assert_eq!(!routes_traces.is_empty(), false);

}

#[test]
fn test_rule_with_method_3() {
    let router = setup_rule_with_method();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,Some(r#"POST"#.to_string()),None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/baz"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_method_4() {
    let router = setup_rule_with_method();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/bar"#), r#"/bar"#.to_string(),None,None,Some(r#"PUT"#.to_string()),None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bor"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_method_5() {
    let router = setup_rule_with_method();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/bar"#), r#"/bar"#.to_string(),None,None,Some(r#"POST"#.to_string()),None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bor"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_with_quotes() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"host-path-query-double-quotes","rank":0,"source":{"host":"example.org","path":"/host-path-query-double-quotes","query":"gender.nl-NL=Dames%22,%22Heren%22,%22Kinderens"},"status_code":301,"target":"/target?gender=Dames&gender=Heren&gender=Kinderen"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_rule_with_quotes_1() {
    let router = setup_rule_with_quotes();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/host-path-query-double-quotes?gender.nl-NL=Dames%22,%22Heren%22,%22Kinderens"#), r#"/host-path-query-double-quotes?gender.nl-NL=Dames%22,%22Heren%22,%22Kinderens"#.to_string(),Some(r#"example.org"#.to_string()),Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 301);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target?gender=Dames&gender=Heren&gender=Kinderen"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_with_response_status_codes() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-404","rank":0,"source":{"path":"/foo","response_status_codes":[404]},"status_code":302,"target":"/baz"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-404-402","rank":0,"source":{"path":"/bar","response_status_codes":[400,402]},"status_code":302,"target":"/bor"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    let route_3: Rule = serde_json::from_str(r#"{"id":"rule-A-404","rank":10,"source":{"path":"/something","response_status_codes":[404]},"status_code":302,"target":"/A-target"}"#).expect("cannot deserialize");
    router.insert(route_3.into_route(&router.config));

    let route_4: Rule = serde_json::from_str(r#"{"id":"rule-B","rank":100,"source":{"path":"/something"},"status_code":302,"target":"/B-target"}"#).expect("cannot deserialize");
    router.insert(route_4.into_route(&router.config));

    router
}


#[test]
fn test_rule_with_response_status_codes_1() {
    let router = setup_rule_with_response_status_codes();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 200;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_response_status_codes_2() {
    let router = setup_rule_with_response_status_codes();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/bar"#), r#"/bar"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 200;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_response_status_codes_3() {
    let router = setup_rule_with_response_status_codes();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 404;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/baz"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_response_status_codes_4() {
    let router = setup_rule_with_response_status_codes();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/bar"#), r#"/bar"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 400;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bor"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_response_status_codes_5() {
    let router = setup_rule_with_response_status_codes();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/bar"#), r#"/bar"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 402;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bor"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_response_status_codes_6() {
    let router = setup_rule_with_response_status_codes();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/something"#), r#"/something"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 404;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/A-target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_response_status_codes_7() {
    let router = setup_rule_with_response_status_codes();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/something"#), r#"/something"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 200;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/B-target"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_response_status_codes_8() {
    let router = setup_rule_with_response_status_codes();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/something"#), r#"/something"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 0);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_rule_with_slash() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"rule-with-slash","rank":0,"source":{"path":"/foo/"},"status_code":302,"target":"/bar/"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    let route_2: Rule = serde_json::from_str(r#"{"id":"rule-without-slash","rank":0,"source":{"path":"/foo"},"status_code":302,"target":"/bar"}"#).expect("cannot deserialize");
    router.insert(route_2.into_route(&router.config));

    router
}


#[test]
fn test_rule_with_slash_1() {
    let router = setup_rule_with_slash();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo"#), r#"/foo"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_rule_with_slash_2() {
    let router = setup_rule_with_slash();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/foo/"#), r#"/foo/"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/bar/"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_variable_marker() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"camelize-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?"}],"rank":0,"source":{"path":"/camelize/from/@marker"},"status_code":302,"target":"/camelize/target/@var_marker_1","variables":[{"name":"var_marker_1","transformers":[{"options":null,"type":"camelize"}],"type":{"marker":"marker"}}]}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_variable_marker_1() {
    let router = setup_variable_marker();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/helloPoney"#), r#"/camelize/from/helloPoney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPoney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_variable_marker_2() {
    let router = setup_variable_marker();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/Hello-poney"#), r#"/camelize/from/Hello-poney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPoney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_variable_marker_3() {
    let router = setup_variable_marker();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/HelloPoney"#), r#"/camelize/from/HelloPoney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPoney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_variable_marker_4() {
    let router = setup_variable_marker();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/hello-pOney"#), r#"/camelize/from/hello-pOney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/helloPOney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_variable_marker_legacy() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"camelize-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?"}],"rank":0,"source":{"path":"/camelize/from/@marker"},"status_code":302,"target":"/camelize/@marker/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_variable_marker_legacy_1() {
    let router = setup_variable_marker_legacy();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/helloPoney"#), r#"/camelize/from/helloPoney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/helloPoney/target/helloPoney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_variable_marker_legacy_1() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"camelize-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?"}],"rank":0,"source":{"host":"","path":"/camelize/from/@marker","query":""},"status_code":302,"target":"/camelize/@marker/target/@marker"}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_variable_marker_legacy_1_1() {
    let router = setup_variable_marker_legacy_1();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/helloPoney"#), r#"/camelize/from/helloPoney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/helloPoney/target/helloPoney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_variable_marker_order() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"camelize-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?"}],"rank":0,"source":{"path":"/camelize/from/@marker"},"status_code":302,"target":"/camelize/@var/@var2/target/@var3","variables":[{"name":"var3","type":"request_host"},{"name":"var2","type":"request_scheme"},{"name":"var","type":{"marker":"marker"}}]}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_variable_marker_order_1() {
    let router = setup_variable_marker_order();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/helloPoney"#), r#"/camelize/from/helloPoney"#.to_string(),Some(r#"test.com"#.to_string()),Some(r#"https"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/helloPoney/https/target/test.com"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_variable_marker_transformer() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"camelize-rule","markers":[{"name":"marker","regex":"([\\p{Ll}\\p{Lu}\\p{Lt}]|\\-)+?","transformers":[{"options":{"from":"0","to":"5"},"type":"slice"}]}],"rank":0,"source":{"path":"/camelize/from/@marker"},"status_code":302,"target":"/camelize/target/@var_marker_1","variables":[{"name":"var_marker_1","transformers":[{"options":null,"type":"uppercase"}],"type":{"marker":"marker"}}]}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_variable_marker_transformer_1() {
    let router = setup_variable_marker_transformer();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/camelize/from/helloPoney"#), r#"/camelize/from/helloPoney"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/camelize/target/HELLO"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_variable_request_header() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"camelize-rule","rank":0,"source":{"path":"/variable/request-header"},"status_code":302,"target":"/target/request-header/@var1","variables":[{"name":"var1","type":{"request_header":{"default":"Foo","name":"X-Request-Header"}}}]}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_variable_request_header_1() {
    let router = setup_variable_request_header();
    let default_config = RouterConfig::default();
    let mut request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/variable/request-header"#), r#"/variable/request-header"#.to_string(),None,None,None,None,None);
    request.add_header(r#"X-Request-Header"#.to_string(), r#"helloPoney"#.to_string(), false);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target/request-header/helloPoney"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_variable_request_header_2() {
    let router = setup_variable_request_header();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/variable/request-header"#), r#"/variable/request-header"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target/request-header/Foo"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_variable_request_host() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"camelize-rule","rank":0,"source":{"path":"/variable/request-header"},"status_code":302,"target":"/target/request-header/@var1","variables":[{"name":"var1","type":"request_host"}]}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_variable_request_host_1() {
    let router = setup_variable_request_host();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/variable/request-header"#), r#"/variable/request-header"#.to_string(),Some(r#"example.com"#.to_string()),None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target/request-header/example.com"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_variable_request_method() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"camelize-rule","rank":0,"source":{"path":"/variable/request-header"},"status_code":302,"target":"/target/request-header/@var1","variables":[{"name":"var1","type":"request_method"}]}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_variable_request_method_1() {
    let router = setup_variable_request_method();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/variable/request-header"#), r#"/variable/request-header"#.to_string(),None,None,Some(r#"GET"#.to_string()),None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target/request-header/GET"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_variable_request_method_2() {
    let router = setup_variable_request_method();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/variable/request-header"#), r#"/variable/request-header"#.to_string(),None,None,Some(r#"POST"#.to_string()),None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target/request-header/POST"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_variable_request_path() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"camelize-rule","rank":0,"source":{"path":"/variable/request-header"},"status_code":302,"target":"/target/request-header@var1","variables":[{"name":"var1","type":"request_path"}]}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_variable_request_path_1() {
    let router = setup_variable_request_path();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/variable/request-header"#), r#"/variable/request-header"#.to_string(),None,None,None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target/request-header/variable/request-header"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


fn setup_variable_request_scheme() -> Router<Rule> {
    let config: RouterConfig = serde_json::from_str(r#"{"always_match_any_host":false,"ignore_header_case":false,"ignore_host_case":false,"ignore_marketing_query_params":true,"ignore_path_and_query_case":false,"marketing_query_params":["utm_source","utm_medium","utm_campaign","utm_term","utm_content"],"pass_marketing_query_params_to_target":true}"#).expect("cannot deserialize");
    let mut router = Router::<Rule>::from_config(config);

    let route_1: Rule = serde_json::from_str(r#"{"id":"camelize-rule","rank":0,"source":{"path":"/variable/request-header"},"status_code":302,"target":"/target/request-header/@var1","variables":[{"name":"var1","type":"request_scheme"}]}"#).expect("cannot deserialize");
    router.insert(route_1.into_route(&router.config));

    router
}


#[test]
fn test_variable_request_scheme_1() {
    let router = setup_variable_request_scheme();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/variable/request-header"#), r#"/variable/request-header"#.to_string(),None,Some(r#"https"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target/request-header/https"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}

#[test]
fn test_variable_request_scheme_2() {
    let router = setup_variable_request_scheme();
    let default_config = RouterConfig::default();
    let request = Request::new(PathAndQueryWithSkipped::from_config(&default_config, r#"/variable/request-header"#), r#"/variable/request-header"#.to_string(),None,Some(r#"http"#.to_string()),None,None,None);
    let request_configured = Request::rebuild_with_config(&router.config, &request);
    let matched = router.match_request(&request_configured);
    let traces = router.trace_request(&request_configured);
    let routes_traces = Trace::<Rule>::get_routes_from_traces(&traces);

    assert_eq!(!matched.is_empty(), true);
    assert_eq!(!routes_traces.is_empty(), true);

    let mut action = Action::from_routes_rule(matched, &request_configured);
    let response_status_code = 0;

    let action_status_code = action.get_status_code(response_status_code, None);
    assert_eq!(action_status_code, 302);
    let headers = action.filter_headers(Vec::new(), response_status_code, false, None);
    assert_eq!(headers.len(), 1);

    let target_header = headers.first().unwrap();
    assert_eq!(target_header.name, "Location");
    assert_eq!(target_header.value, r#"/target/request-header/http"#);
    assert_eq!(action.should_log_request(true, response_status_code), true);
}


}
