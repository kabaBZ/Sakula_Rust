#[path = "crawler/mod.rs"]
mod crawler;

#[path = "request/mod.rs"]
mod request;

use crate::crawler::sakula::*;
use crate::request::my_request::*;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use std::collections::HashMap;

#[allow(dead_code)]
fn check_headers() -> Result<()> {
    let mut sakula_crawler = Sakula::new();
    let ip = sakula_crawler
        .req
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
    // check_headers()?;
    let mut sakula_dfheaders = HeaderMap::new();
    sakula_dfheaders.insert("client", HeaderValue::from_str("Rust").unwrap());

    let mut sakula_crawler = Sakula::new();

    let result: SearchResult = sakula_crawler.search("电".to_string())?;
    let eps: SelectedMovie = sakula_crawler.select_movie(result)?;
    let m3u8_map: HashMap<usize, String> = sakula_crawler.select_ep(eps)?;
    sakula_crawler.download(m3u8_map).expect("下载报错");

    Ok(())
}
