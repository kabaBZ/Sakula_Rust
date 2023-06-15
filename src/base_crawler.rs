use crate::{my_request::Request, MyRequests};
use error_chain::error_chain;
use reqwest::header::HeaderMap;
use reqwest::Method;
use scraper::{Html, Selector};
use std::collections::HashMap;
error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}

pub enum StationName {
    Sakula,
    Default,
}

pub struct Station {
    pub name: StationName,
    pub host: String, //  "http://www.yinghuacd.com"
    pub req: MyRequests,
}

pub trait Crawl {
    fn search_in_sakula(&mut self, keyword: String) -> Result<()>;
    fn search(&mut self, keyword: String) -> Result<()>;
    fn do_not_search(&self) -> Result<()>;
    fn download();
}

impl Crawl for Station {
    fn do_not_search(&self) -> Result<()> {
        println!("do not search");
        Ok(())
    }
    fn search_in_sakula(&mut self, keyword: String) -> Result<()> {
        // self.searchResult = SearchResult(resultNames, resultHrefs).data
        let search_json = HashMap::from([
            ("m", "search"),
            ("c", "index"),
            ("a", "init"),
            ("q", &keyword),
        ]);
        let search_url = self.host.clone() + &format!("/search/{}", &keyword);
        let res = self
            .req
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

    fn search(&mut self, keyword: String) -> Result<()> {
        match self.name {
            StationName::Sakula => self.search_in_sakula(keyword),
            _ => self.do_not_search(),
        }
    }
    fn download() {}
}
