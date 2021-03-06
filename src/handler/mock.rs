use hyper::Body;

use crate::method::{self, MethodFilter};
use crate::{config, response};

#[derive(Debug)]
pub struct MockHandler {
    config: config::MockRoute,
}

pub fn default_method_filter() -> Box<dyn MethodFilter> {
    method::any()
}

impl MockHandler {
    pub fn new(config: config::MockRoute) -> Self {
        MockHandler { config }
    }

    pub async fn handle(
        &self,
        _: http::Request<Body>,
    ) -> Result<http::Response<Body>, (http::Request<Body>, http::Response<Body>)> {
        match &self.config.body {
            Some(value) => {
                let mut response = response::json(value);
                *response.status_mut() = self.config.status;
                Ok(response)
            }
            None => Ok(response::from_status(self.config.status)),
        }
    }
}
