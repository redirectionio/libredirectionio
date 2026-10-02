use std::{cell::RefCell, rc::Rc};

use serde::Serialize;

use crate::{
    action::{Action, UnitTrace},
    api::{Example, RedirectionLoop, Rule},
    http::{Header, Request},
    router::{Router, Trace},
};

#[derive(Serialize, Debug, Clone)]
pub struct RunExample {
    pub request: Request,
    pub(crate) unit_trace: UnitTrace,
    pub(crate) backend_status_code: u16,
    pub(crate) response: RunResponse,
    pub(crate) should_log_request: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) log_tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) backend_request_headers: Option<Vec<Header>>,
    pub(crate) redirection_loop: Option<RedirectionLoop>,
    pub(crate) match_traces: Vec<Trace<Rule>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct RunResponse {
    pub(crate) status_code: u16,
    pub(crate) headers: Vec<Header>,
    pub(crate) body: String,
}

impl RunExample {
    pub fn new(router: &Router<Rule>, example: &Example) -> Result<Self, http::Error> {
        let request = Request::from_example(&router.config, example)?;
        let routes = router.match_request(&request);
        let unit_trace = Rc::new(RefCell::new(UnitTrace::default()));

        let mut action = Action::from_routes_rule(routes, &request, Some(unit_trace.clone()));

        // A replay reaches no backend, but the request would have been sent to this one: the
        // backend switch is applied, and its unit has to be traced as such like every other.
        action.get_peer(Some(unit_trace.clone()));

        let backend_request_headers = if action.has_request_header_filters() {
            Some(action.filter_request_headers(request.headers.clone(), Some(unit_trace.clone())))
        } else {
            None
        };

        let action_status_code = action.get_status_code(0, Some(unit_trace.clone()));
        let (final_status_code, backend_status_code) = if action_status_code != 0 {
            (action_status_code, action_status_code)
        } else {
            // We call the backend and get a response code
            let backend_status_code = example.response_status_code.unwrap_or(200);
            let final_status_code = action.get_status_code(backend_status_code, Some(unit_trace.clone()));

            (final_status_code, backend_status_code)
        };

        let headers = action.filter_headers(Vec::new(), backend_status_code, false, Some(unit_trace.clone()));

        let mut body = example.response_body.as_deref().unwrap_or(
            "<!DOCTYPE html>
<html>
    <head>
    </head>
    <body>
    </body>
</html>",
        );

        let mut b1;
        if let Some(mut body_filter) = action.create_filter_body(backend_status_code, &[], Some(unit_trace.clone())) {
            b1 = body_filter.filter(body.into(), Some(unit_trace.clone()));
            let b2 = body_filter.end(Some(unit_trace.clone()));
            b1.extend(b2);
            body = std::str::from_utf8(&b1).unwrap();
        }

        let should_log_request = action.should_log_request(true, final_status_code, Some(unit_trace.clone()));
        let response_status_code = if final_status_code != 0 {
            final_status_code
        } else {
            backend_status_code
        };
        let log_tags = action.get_log_tags(response_status_code, Some(unit_trace.clone()));
        let mut unit_trace = unit_trace.take();
        unit_trace.squash_with_target_unit_traces();

        Ok(RunExample {
            request,
            unit_trace,
            backend_status_code,
            response: RunResponse {
                status_code: final_status_code,
                headers,
                body: body.to_string(),
            },
            should_log_request,
            log_tags,
            backend_request_headers,
            redirection_loop: None,
            match_traces: vec![],
        })
    }

    pub fn with_redirection_loop(&mut self, router: &Router<Rule>, max_hops: u8, example: &Example, project_domains: Vec<String>) {
        self.redirection_loop = Some(RedirectionLoop::from_example(router, max_hops, example, project_domains));
    }

    pub fn with_match_traces(&mut self, router: &Router<Rule>) {
        self.match_traces = router.trace_request(&self.request);
    }
}
