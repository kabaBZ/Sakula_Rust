use error_chain::error_chain;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
// use std::collections::HashMap;
pub mod RequestHeaders;
pub mod my_request;
use my_request::{MyRequests, Request};

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}

fn main() -> Result<()> {
    let sakula_request = MyRequests {
        session: Client::new(),
        headers: HeaderMap::new(),
    };
    let ip = sakula_request
        .get(
            "Http://myip.top".to_owned(),
            vec![("client", "rust")],
            HeaderMap::new(),
        )?
        .text()?;

    println!("{}", ip);
    Ok(())
}
