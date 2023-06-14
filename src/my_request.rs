use reqwest::blocking::Client;
use reqwest::blocking::Response;
// use reqwest::header;
use reqwest::header::HeaderMap;
// use std::collections::HashMap;
// use std::io::Error;
use url::Url;

pub struct MyRequests {
    pub session: Client,
    pub headers: HeaderMap,
    // proxy: HashMap<String, String>,
}

pub trait SetHeaders {
    fn new(&mut self);
}

impl SetHeaders for MyRequests {
    fn new(&mut self) {
        self.session = Client::builder()
            .default_headers(self.headers.clone())
            .build()
            .unwrap();
    }
}

pub trait Request {
    fn get(
        &self,
        url: String,
        params: Vec<(&str, &str)>,
        add_headers: HeaderMap,
    ) -> Result<Response, reqwest::Error>;
    // fn post<T>(&self, url: String, data: T, _kargs: HashMap<String, String>);
}

impl Request for MyRequests {
    fn get(
        &self,
        url: String,
        params: Vec<(&str, &str)>,
        add_headers: HeaderMap,
    ) -> Result<Response, reqwest::Error> {
        let p_url = Url::parse_with_params(&url, &params).unwrap();
        self.session.get(p_url).headers(add_headers).send()
    }
    // fn post<T>(&self, url: String, data: T, _kargs: HashMap<String, String>) {
    //     println!("123");
    // }
}
