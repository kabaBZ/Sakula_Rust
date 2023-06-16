use reqwest::header::HeaderMap;

pub trait Headers {
    fn get_default_headers(&self) -> HeaderMap;
}
