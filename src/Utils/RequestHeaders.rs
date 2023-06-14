use error_chain::error_chain;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}

fn main() -> Result<()> {
    let url = Url::parse_with_params(
        "http://httpbin.org/headers",
        &[("lang", "rust"), ("browser", "servo")],
    )?;

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_str("Rust-test").unwrap());
    headers.insert(
        "Authorization",
        HeaderValue::from_str("Bearer DEadBEEfc001cAFeEDEcafBAd").unwrap(),
    );
    headers.insert(
        "X-Powered-By",
        HeaderValue::from_str("Guybrush Threepwood").unwrap(),
    );

    let response = Client::new().get(url).headers(headers).send()?;

    let res_url = response.url().clone();

    let out: HeadersEcho = serde_json::from_str(&response.text()?).unwrap();

    assert_eq!(
        out.headers["Authorization"],
        "Bearer DEadBEEfc001cAFeEDEcafBAd"
    );
    assert_eq!(out.headers["User-Agent"], "Rust-test");
    assert_eq!(out.headers["X-Powered-By"], "Guybrush Threepwood");
    assert_eq!(
        res_url.as_str(),
        "http://httpbin.org/headers?lang=rust&browser=servo"
    );

    println!("{:?}", out);
    Ok(())
}
