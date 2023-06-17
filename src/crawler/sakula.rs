use crate::request::my_request::*;

use error_chain::error_chain;
// use reqwest::header::HeaderMap;
// use std::sync::Arc;
error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}

pub enum StationName {
    Sakula,
}

pub struct SearchResult {
    pub names: Vec<String>,
    pub hrefs: Vec<String>,
}

pub struct SelectedMovie {
    pub name: String,
    pub href: String,
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
    fn search(&mut self, keyword: String) -> Result<SearchResult>;
    fn select_movie(&mut self, result: SearchResult) -> Result<SelectedMovie>;
    fn select_ep(&mut self, movie: SelectedMovie) -> Result<()>;
    fn download();
}
