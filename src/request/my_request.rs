use reqwest::blocking::Client;
use reqwest::blocking::RequestBuilder;
use reqwest::header::HeaderMap;
use reqwest::Method;
use std::collections::HashMap;

pub struct MyRequests {
    pub session: Client,
    pub headers: HeaderMap,
    // proxy: HashMap<String, String>,
}

pub trait Init {
    fn new() -> MyRequests;
}

pub trait Request {
    fn build_request(
        &mut self,
        method: Method,
        url: String,
        params: HashMap<&str, &str>,
        add_headers: HeaderMap,
    ) -> RequestBuilder;
    fn update_headers(&mut self, header: HeaderMap) -> ();
    fn set_headers(&mut self) -> ();
}

pub trait SakulaHeaders: Request {
    fn get_default_headers(&self) -> HeaderMap;
}
