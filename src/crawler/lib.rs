use crate::crawler::sakula::*;
use crate::headers::sakula::*;
use crate::request::my_request::*;
use crate::request::sakula::*;
use reqwest::header::HeaderMap;
use reqwest::Method;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::io::stdin;
use std::thread::{self, JoinHandle}; // 引入thread

impl Crawl for Sakula {
    fn search(&mut self, keyword: String) -> Result<SearchResult> {
        // self.searchResult = SearchResult(resultNames, resultHrefs).data
        let search_json = HashMap::from([
            ("m", "search"),
            ("c", "index"),
            ("a", "init"),
            ("q", &keyword),
        ]);
        let search_url = self.host.clone() + &format!("/search/{}", &keyword);
        let res = self
            .build_request(Method::POST, search_url, HashMap::new(), HeaderMap::new())
            .json(&search_json)
            .send()?;
        let response_text = res.text()?;
        // println!("{:#?}", response_text);
        let search_document = Html::parse_document(&response_text);
        // resultHrefs = result.xpath('.//div[@class="lpic"]//li/a/@href')
        // resultNames = result.xpath('.//div[@class="lpic"]//li/a/img/@alt')
        let result_hrefs_selector = Selector::parse("div.lpic li > a").unwrap();
        let result_names_selector = Selector::parse("div.lpic li a img").unwrap();

        let mut result_hrefs: Vec<String> = vec![];
        for res in search_document.select(&result_hrefs_selector) {
            result_hrefs.push(res.value().attr("href").unwrap().to_string())
        }

        let mut result_names: Vec<String> = vec![];
        for res in search_document.select(&result_names_selector) {
            result_names.push(res.value().attr("alt").unwrap().to_string())
        }
        println!("{:#?}, {:#?}", result_hrefs, result_names);
        Ok(SearchResult {
            names: result_names,
            hrefs: result_hrefs,
        })
    }

    fn select_movie(&mut self, result: SearchResult) -> Result<SelectedMovie> {
        for (i, name) in result.names.iter().enumerate() {
            println!("{}.{}", i + 1, name)
        }
        println!("请输入数字序号选择：");
        let mut num_s = String::new();
        let mut num: usize;
        loop {
            stdin().read_line(&mut num_s).expect("Failed to real line!");
            println!("input:{}", num_s);
            num = num_s.trim().parse().expect("请输入存在的数字序号!");
            match num {
                num if num > result.names.len() => println!("请输入存在的序号!"),
                _ => {
                    println!("你选择的序号是{},对应名称为{}", num, result.names[num - 1]);
                    break;
                }
            }
        }
        Ok(SelectedMovie {
            name: result.names[num - 1].clone(),
            href: result.hrefs[num - 1].clone(),
        })
    }

    fn select_ep(&mut self, movie: SelectedMovie) -> Result<()> {
        let page_text = self
            .build_request(
                Method::GET,
                self.host.clone() + &movie.href,
                HashMap::new(),
                HeaderMap::new(),
            )
            .send()?
            .text()?;
        let page_document = Html::parse_document(&page_text);
        // Ep_hrefs = Ep_page_res.xpath('//div[@class="movurl"]/ul/li/a/@href')
        let ep_hrefs_selector = Selector::parse("div.movurl > ul > li > a").unwrap();
        let mut ep_hrefs: Vec<String> = vec![];
        for res in page_document.select(&ep_hrefs_selector) {
            ep_hrefs.push(res.value().attr("href").unwrap().to_string())
        }
        for (i, href) in ep_hrefs.iter().enumerate() {
            println!("EP{}, href:{}", i + 1, href)
        }
        println!("请输入数字序号选择(0为全部下载):");
        let ep_num = 0;
        let choosen_ep = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // 开线程获取m3u8的url
        let mut thread_pool: Vec<JoinHandle<()>> = vec![];
        for (i, href) in ep_hrefs.iter().enumerate() {
            if choosen_ep.contains(&i) {
                let t: JoinHandle<()> = thread::spawn(move || {});
                thread_pool.push(t)
            };
        }
        for t in thread_pool.into_iter() {
            t.join().unwrap();
        }
        Ok(())
    }

    fn download() {}
    fn set_headers(&mut self) -> () {
        self.req.headers = self.get_default_headers();
    }
    fn update_headers(&mut self, header: HeaderMap) -> () {
        self.req.headers.extend(header);
    }
}

impl New for Sakula {
    fn new() -> Sakula {
        Sakula {
            name: StationName::Sakula,
            host: "http://www.yinghuacd.com".to_string(),
            req: MyRequests::new(),
        }
    }
}
