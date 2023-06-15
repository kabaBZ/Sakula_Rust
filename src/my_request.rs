use chrono::prelude::*;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use std::collections::HashMap;
use url::Url;

pub struct MyRequests {
    pub session: Client,
    pub reqheaders: ReqHeaders,
    // proxy: HashMap<String, String>,
}

pub enum Mode {
    Sakula,
    Default,
}

pub struct ReqHeaders {
    pub headers: HeaderMap,
    pub mode: Mode,
}

pub trait Init {
    fn new(mode: Mode) -> MyRequests;
}

impl Init for MyRequests {
    fn new(mode: Mode) -> MyRequests {
        MyRequests {
            session: Client::builder().build().unwrap(),
            reqheaders: ReqHeaders {
                headers: HeaderMap::new(),
                mode,
            },
        }
    }
}

trait Headers {
    fn set_headers(&mut self) -> ();
    fn update_headers(&mut self, header: HeaderMap) -> ();
    fn get_default_headers(&self) -> HeaderMap;
    fn get_sakula_headers(&self) -> HeaderMap;
}

impl Headers for ReqHeaders {
    fn set_headers(&mut self) -> () {
        match self.mode {
            Mode::Default => self.headers = self.get_default_headers(),
            Mode::Sakula => self.headers = self.get_sakula_headers(),
        }
    }
    fn update_headers(&mut self, header: HeaderMap) -> () {
        self.headers.extend(header);
    }
    fn get_default_headers(&self) -> HeaderMap {
        HeaderMap::new()
    }
    fn get_sakula_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("NewHeader", HeaderValue::from_str("Sakula").unwrap());
        headers.insert(
            "TimeStamp",
            HeaderValue::from_str(&Utc::now().timestamp().to_string()).unwrap(),
        );

        headers
    }
}

pub trait Request {
    fn do_request(
        &mut self,
        method: Method,
        url: String,
        params: HashMap<&str, &str>,
        add_headers: HeaderMap,
    ) -> Result<Response, reqwest::Error>;
}

impl Request for MyRequests {
    fn do_request(
        &mut self,
        method: Method,
        url: String,
        params: HashMap<&str, &str>,
        add_headers: HeaderMap,
    ) -> Result<Response, reqwest::Error> {
        let p_url = Url::parse_with_params(&url, &params).unwrap();
        self.reqheaders.set_headers();
        self.reqheaders.update_headers(add_headers);
        self.session
            .request(method, p_url)
            .headers(self.reqheaders.headers.clone())
            .send()
    }
}
