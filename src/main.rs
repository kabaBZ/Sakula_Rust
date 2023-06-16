#[path = "crawler/mod.rs"]
mod crawler;

#[path = "headers/mod.rs"]
mod headers;

#[path = "request/mod.rs"]
mod request;

#[path = "tools/mod.rs"]
mod tools;

use crate::crawler::sakula::*;
use crate::request::sakula::*;

use reqwest::header::HeaderMap;
use reqwest::Method;
use std::collections::HashMap;

fn check_headers() -> Result<()> {
    let mut sakula_crawler = Sakula::new();
    let ip = sakula_crawler
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

fn main() -> Result<()> {
    check_headers()
    // let mut sakula_dfheaders = HeaderMap::new();
    // sakula_dfheaders.insert("client", HeaderValue::from_str("Rust").unwrap());

    // let sakula_request = MyRequests::new(Mode::Sakula);

    // let mut sakula_crawler = Station {
    //     name: StationName::Sakula,
    //     host: "http://www.yinghuacd.com".to_string(),
    //     req: sakula_request,
    // };
    // sakula_crawler.search("电".to_string())
}
