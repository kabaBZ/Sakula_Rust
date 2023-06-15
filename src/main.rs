use error_chain::error_chain;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use std::collections::HashMap;
pub mod my_request;
use my_request::{Mode, MyRequests, Request};

use crate::my_request::Init;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}

fn main() -> Result<()> {
    let mut sakula_dfheaders = HeaderMap::new();
    sakula_dfheaders.insert("client", HeaderValue::from_str("Rust").unwrap());

    let mut sakula_request = MyRequests::new(Mode::Sakula);

    let ip = sakula_request
        .do_request(
            Method::GET,
            "http://httpbin.org/headers".to_owned(),
            HashMap::from([("addurl", "rust")]),
            HeaderMap::new(),
        )?
        .text()?;
    // .url()
    // .to_string();

    println!("{}", ip);
    Ok(())
}
