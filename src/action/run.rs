use serde::Serialize;

use crate::{
    action::{Action, UnitTrace},
    api::{Example, RedirectionLoop, Rule},
    http::{Header, Request},
    router::{Router, Trace},
};

#[derive(Serialize, Debug, Clone)]
pub struct ExampleRun {
    pub(crate) request: Request,
    pub(crate) unit_trace: UnitTrace,
    pub(crate) backend_status_code: u16,
    pub(crate) response: RunResponse,
    pub(crate) should_log_request: bool,
    pub(crate) redirection_loop: Option<RedirectionLoop>,
    #[cfg(feature = "router")]
    pub(crate) match_traces: Vec<Trace<Rule>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct RunResponse {
    pub(crate) status_code: u16,
    pub(crate) headers: Vec<Header>,
    pub(crate) body: String,
}

impl ExampleRun {
    #[cfg(feature = "router")]
    pub fn new(router: &Router<Rule>, example: &Example) -> Result<Self, http::Error> {
        let request = Request::from_example(&router.config, example)?;
        let routes = router.match_request(&request);
        let mut unit_trace = UnitTrace::default();

        let mut action = Action::from_routes_rule(routes, &request, Some(&mut unit_trace));

        let action_status_code = action.get_status_code(0, Some(&mut unit_trace));
        let (final_status_code, backend_status_code) = if action_status_code != 0 {
            (action_status_code, action_status_code)
        } else {
            // We call the backend and get a response code
            let backend_status_code = example.response_status_code.unwrap_or(200);
            let final_status_code = action.get_status_code(backend_status_code, Some(&mut unit_trace));

            (final_status_code, backend_status_code)
        };

        let headers = action.filter_headers(Vec::new(), backend_status_code, false, Some(&mut unit_trace));

        let mut body = "<!DOCTYPE html>
<html>
    <head>
    </head>
    <body>
    </body>
</html>";

        let mut b1;
        if let Some(mut body_filter) = action.create_filter_body(backend_status_code, &[]) {
            b1 = body_filter.filter(body.into(), Some(&mut unit_trace));
            let b2 = body_filter.end(Some(&mut unit_trace));
            b1.extend(b2);
            body = std::str::from_utf8(&b1).unwrap();
        }

        let should_log_request = action.should_log_request(true, final_status_code, Some(&mut unit_trace));

        unit_trace.squash_with_target_unit_traces();

        Ok(ExampleRun {
            request,
            unit_trace,
            backend_status_code,
            response: RunResponse {
                status_code: final_status_code,
                headers,
                body: body.to_string(),
            },
            should_log_request,
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
