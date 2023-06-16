use crate::crawler::sakula::Sakula;
use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};

pub trait Headers {
    fn get_default_headers(&self) -> HeaderMap;
}

impl Headers for Sakula {
    fn get_default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("NewHeader", HeaderValue::from_str("Sakula").unwrap());
        headers.insert(
            "TimeStamp",
            HeaderValue::from_str(&Utc::now().timestamp().to_string()).unwrap(),
        );

        headers
    }
}
