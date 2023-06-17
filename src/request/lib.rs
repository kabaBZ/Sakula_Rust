use crate::request::my_request::*;
use chrono::Utc;
use reqwest::blocking::Client;
use reqwest::blocking::RequestBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use std::collections::HashMap;
use url::Url;

impl Init for MyRequests {
    fn new() -> MyRequests {
        MyRequests {
            session: Client::builder().build().unwrap(),
            headers: HeaderMap::new(),
            // local_storage = HashMap::new(),
            // cookies: HashMap::new()
        }
    }
}

impl Request for MyRequests {
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
        self.session
            .request(method, p_url)
            .headers(self.headers.clone())
    }
    fn update_headers(&mut self, header: HeaderMap) -> () {
        self.headers.extend(header);
    }
    fn set_headers(&mut self) -> () {
        self.headers = self.get_default_headers();
    }
}

impl SakulaHeaders for MyRequests {
    fn get_default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("NewHeader", HeaderValue::from_str("Sakula").unwrap());
        headers.insert(
            "TimeStamp",
            HeaderValue::from_str(&Utc::now().timestamp().to_string()).unwrap(),
        );

        headers
    }
}
