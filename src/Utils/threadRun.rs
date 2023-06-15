use error_chain::error_chain;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
// use std::collections::HashMap;
use std::thread::{self, JoinHandle}; // 引入thread
use std::time::Duration; // 引入time::Duration，用来创建时间类型数据
                         // pub mod RequestHeaders;
pub mod my_request;
use my_request::{MyRequests, Request};

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        UrlParse(url::ParseError);
    }
}

fn main() -> Result<()> {
    let x: u64 = 1;
    let closure_slision = move || -> u64 { x };
    let mut thread_pool: Vec<JoinHandle<()>> = vec![];
    for i in 1..10 {
        let t: JoinHandle<()> = thread::spawn(move || {
            thread::sleep(Duration::from_secs(closure_slision()));
            println!("Thread No.{}", i);
        });
        thread_pool.push(t)
    }
    for t in thread_pool.into_iter() {
        t.join().unwrap();
    }
    Ok(())
}
