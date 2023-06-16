use crate::request::my_request::*;

use reqwest::header::HeaderMap;

use error_chain::error_chain;
error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}

pub enum StationName {
    Sakula,
}

pub struct Sakula {
    pub name: StationName,
    pub host: String, //  "http://www.yinghuacd.com"
    pub req: MyRequests,
}

pub trait New {
    fn new() -> Sakula;
}

pub trait Crawl {
    fn search(&mut self, keyword: String) -> Result<()>;
    fn download();
    fn set_headers(&mut self) -> ();
    fn update_headers(&mut self, header: HeaderMap) -> ();
}
