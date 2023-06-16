use crate::crawler::sakula::Crawl;
use crate::crawler::sakula::Sakula;
use reqwest::blocking::RequestBuilder;
use reqwest::header::HeaderMap;
use reqwest::Method;
use std::collections::HashMap;
use url::Url;

pub trait Request {
    fn build_request(
        &mut self,
        method: Method,
        url: String,
        params: HashMap<&str, &str>,
        add_headers: HeaderMap,
    ) -> RequestBuilder;
}

impl Request for Sakula {
    fn build_request(
        &mut self,
        method: Method,
        url: String,
        params: HashMap<&str, &str>,
        add_headers: HeaderMap,
    ) -> RequestBuilder {
        let p_url = Url::parse_with_params(&url, &params).unwrap();
        self.set_headers();
        self.update_headers(add_headers);
        self.req
            .session
            .request(method, p_url)
            .headers(self.req.headers.clone())
    }
}
