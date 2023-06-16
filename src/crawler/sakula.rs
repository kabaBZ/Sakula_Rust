use crate::headers::sakula::Headers;
use crate::request::sakula::Request;
use crate::tools::request::MyRequests;
use crate::tools::request::*;
use reqwest::header::HeaderMap;
use reqwest::Method;
use scraper::{Html, Selector};
use std::collections::HashMap;

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

impl New for Sakula {
    fn new() -> Sakula {
        Sakula {
            name: StationName::Sakula,
            host: "http://www.yinghuacd.com".to_string(),
            req: MyRequests::new(),
        }
    }
}

pub trait Crawl {
    fn search(&mut self, keyword: String) -> Result<()>;
    fn download();
    fn set_headers(&mut self) -> ();
    fn update_headers(&mut self, header: HeaderMap) -> ();
}

impl Crawl for Sakula {
    fn search(&mut self, keyword: String) -> Result<()> {
        // self.searchResult = SearchResult(resultNames, resultHrefs).data
        let search_json = HashMap::from([
            ("m", "search"),
            ("c", "index"),
            ("a", "init"),
            ("q", &keyword),
        ]);
        let search_url = self.host.clone() + &format!("/search/{}", &keyword);
        let res = self
            .build_request(Method::POST, search_url, HashMap::new(), HeaderMap::new())
            .json(&search_json)
            .send()?;
        let response_text = res.text()?;
        // println!("{:#?}", response_text);
        let search_document = Html::parse_document(&response_text);
        // resultHrefs = result.xpath('.//div[@class="lpic"]//li/a/@href')
        // resultNames = result.xpath('.//div[@class="lpic"]//li/a/img/@alt')
        let result_hrefs_selector = Selector::parse("div.lpic li > a").unwrap();
        let result_names_selector = Selector::parse("div.lpic li a img").unwrap();

        let mut result_hrefs: Vec<&str> = vec![];
        for res in search_document.select(&result_hrefs_selector) {
            result_hrefs.push(res.value().attr("href").unwrap())
        }

        let mut result_names: Vec<&str> = vec![];
        for res in search_document.select(&result_names_selector) {
            result_names.push(res.value().attr("alt").unwrap())
        }
        println!("{:#?}, {:#?}", result_hrefs, result_names);

        Ok(())
    }

    fn download() {}
    fn set_headers(&mut self) -> () {
        self.req.headers = self.get_default_headers();
    }
    fn update_headers(&mut self, header: HeaderMap) -> () {
        self.req.headers.extend(header);
    }
}
