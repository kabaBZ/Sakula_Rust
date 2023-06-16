use reqwest::blocking::Client;
use reqwest::header::HeaderMap;

pub struct MyRequests {
    pub session: Client,
    pub headers: HeaderMap,
    // proxy: HashMap<String, String>,
}

pub trait Init {
    fn new() -> MyRequests;
}

impl Init for MyRequests {
    fn new() -> MyRequests {
        MyRequests {
            session: Client::builder().build().unwrap(),
            headers: HeaderMap::new(),
        }
    }
}
