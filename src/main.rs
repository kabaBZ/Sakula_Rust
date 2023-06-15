// use error_chain::error_chain;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use std::collections::HashMap;
pub mod base_crawler;
pub mod my_request;
use my_request::{Mode, MyRequests, Request}; // , Request

use crate::base_crawler::{Crawl, Station, StationName};
use crate::my_request::Init;

fn check_headers() -> Result<(), my_request::Error> {
    let mut sakula_dfheaders = HeaderMap::new();
    sakula_dfheaders.insert("client", HeaderValue::from_str("Rust").unwrap());

    let mut sakula_request = MyRequests::new(Mode::Sakula);

    let ip = sakula_request
        .build_request(
            Method::GET,
            "http://httpbin.org/headers".to_owned(),
            HashMap::from([("addurl", "rust")]),
            HeaderMap::new(),
        )
        .send()?
        .text()?;
    // .url()
    // .to_string();

    println!("{}", ip);
    Ok(())
}

fn main() -> Result<(), base_crawler::Error> {
    check_headers().unwrap();
    let mut sakula_dfheaders = HeaderMap::new();
    sakula_dfheaders.insert("client", HeaderValue::from_str("Rust").unwrap());

    let sakula_request = MyRequests::new(Mode::Sakula);

    let mut sakula_crawler = Station {
        name: StationName::Sakula,
        host: "http://www.yinghuacd.com".to_string(),
        req: sakula_request,
    };
    sakula_crawler.search("电".to_string())
}
