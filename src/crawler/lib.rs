use crate::crawler::sakula::*;
use crate::request::my_request::{Init, MyRequests, Request};
use regex::Regex;
use reqwest::header::HeaderMap;
use reqwest::Method;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle}; // 引入thread
use std::vec;

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
            .req
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

    fn select_ep(&mut self, movie: SelectedMovie) -> Result<HashMap<usize, String>> {
        self.movie_name = movie.name;
        let page_text = self
            .req
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
        for (i, href) in ep_hrefs.clone().iter().enumerate() {
            println!("EP{}, href:{}", i + 1, href)
        }

        // TODO 增加选择
        // println!("请输入数字序号选择(0为全部下载):");
        // let ep_num = 0;
        let choosen_ep = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        // 开线程获取m3u8的url
        let mut thread_pool: Vec<JoinHandle<()>> = vec![];
        let request = Arc::new(Mutex::new(self.req.clone()));
        let host = Arc::new(Mutex::new(self.host.clone()));
        let m3u8_map = Arc::new(Mutex::new(HashMap::new()));

        for i in 0..ep_hrefs.iter().len() {
            let ep_hrefs_clone = ep_hrefs.clone();
            if choosen_ep.contains(&i) {
                let thread_request = Arc::clone(&request);
                let thread_host = Arc::clone(&host);
                let thread_m3u8_map = Arc::clone(&m3u8_map);
                let t: JoinHandle<()> = thread::spawn(move || {
                    let mut thread = thread_request.lock().unwrap();
                    let sakula_host = thread_host.lock().unwrap();
                    let ep_page = thread
                        .build_request(
                            Method::GET,
                            sakula_host.to_string() + &ep_hrefs_clone[i],
                            HashMap::new(),
                            HeaderMap::new(),
                        )
                        .send()
                        .unwrap()
                        .text()
                        .unwrap();
                    // 提取m3u8页面url  '//div[@id="playbox"]/@data-vid'
                    let ep_document = Html::parse_document(&ep_page);
                    let m3u8_selector = Selector::parse("#playbox[data-vid]").unwrap();
                    let mut m3u8_page_url = "".to_string();
                    if let Some(playbox_div) = ep_document.select(&m3u8_selector).next() {
                        m3u8_page_url = playbox_div.value().attr("data-vid").unwrap().to_string();
                    }
                    let mut m3u8_params = HashMap::new();
                    m3u8_params.insert("vid", m3u8_page_url.as_str());
                    let m3u8_page = thread
                        .build_request(
                            Method::GET,
                            "https://tup.yinghuacd.com/".to_string(),
                            m3u8_params,
                            HeaderMap::new(),
                        )
                        .send()
                        .unwrap()
                        .text()
                        .unwrap();
                    // 正则提取m3u8URL
                    let re = Regex::new(r#"url:\s*"([^"]*)","#).unwrap();
                    let mut m3u8_map = thread_m3u8_map.lock().unwrap();
                    if let Some(capture) = re.captures(&m3u8_page) {
                        let m3u8_url = capture.get(1).unwrap().as_str().to_string();
                        m3u8_map.insert(&i + 1, m3u8_url);
                    }
                });
                thread_pool.push(t);
            };
        }
        for t in thread_pool.into_iter() {
            t.join().unwrap();
        }
        let m3u8 = m3u8_map.lock().unwrap().clone();
        Ok(m3u8)
    }

    fn download(&mut self, m3u8_map: HashMap<usize, String>) -> Result<()> {
        // 开线程下载分割M3U8
        let mut thread_pool: Vec<JoinHandle<()>> = vec![];
        let request = Arc::new(Mutex::new(self.req.clone()));
        let download_chunks: Arc<Mutex<HashMap<usize, Vec<Vec<String>>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        for (k, v) in m3u8_map {
            let thread_request = Arc::clone(&request);
            let thread_dl_chunks = Arc::clone(&download_chunks);

            let t: std::thread::JoinHandle<()> = std::thread::spawn(move || {
                let mut thread_req = thread_request.lock().unwrap();
                let mut thread_chunks = thread_dl_chunks.lock().unwrap();
                let m3u8_content = thread_req
                    .build_request(Method::GET, v, HashMap::new(), HeaderMap::new())
                    .send()
                    .expect("failed to send request")
                    .text()
                    .expect("failed to convert response to text");

                let urls: Vec<String> = m3u8_content
                    .lines()
                    .filter(|line| line.starts_with("http"))
                    .map(|url| url.to_string())
                    .collect();

                // 打印链接
                // println!("{}, {:#?}", k, urls);
                fn divide_into_n_strands(lst: &[String], n: usize) -> Vec<Vec<String>> {
                    let length = lst.len();
                    let sublist_length = length / n as usize;
                    let remainder = length % n as usize;

                    let mut sublists = Vec::new();
                    let mut start = 0;

                    for i in 0..n {
                        let sublist_size = sublist_length + if i < remainder { 1 } else { 0 };
                        let end = start + sublist_size;
                        if end > lst.len() {
                            break;
                        }
                        sublists.push(lst[start..end].to_vec());
                        start = end;
                    }
                    sublists
                }
                thread_chunks.insert(k, divide_into_n_strands(&urls, 4));
            });
            thread_pool.push(t)
        }
        for t in thread_pool {
            t.join().unwrap();
        }
        thread_pool = vec![];
        // 每集一个线程，每个线程开4个子线程下载
        let download_chunks = download_chunks.lock().unwrap();
        for (k, v) in download_chunks.iter() {
            let thread_request = Arc::clone(&request);
            let url_chunks = v.clone();
            let ep = k.clone();
            let t: std::thread::JoinHandle<()> = std::thread::spawn(move || {
                let thread_request = Arc::clone(&thread_request);
                let mut sub_thread = vec![];
                let mut sub_thread_data = HashMap::new();
                for (index, chunk) in url_chunks.iter().enumerate() {
                    let sub_thread_request = Arc::clone(&thread_request);
                    let dl_t = std::thread::spawn(move || {
                        let mut thread_req = sub_thread_request.lock().unwrap();
                        let data: &[u8] = &vec![];
                        for url in chunk {
                            let data_part = thread_req
                                .build_request(
                                    Method::GET,
                                    url.to_string(),
                                    HashMap::new(),
                                    HeaderMap::new(),
                                )
                                .send()
                                .expect("failed to send request")
                                .text()
                                .expect("failed to convert response to text")
                                .as_bytes();
                            data = data. .concat(data_part);
                        }
                        sub_thread_data.insert(index, data);
                    });
                    sub_thread.push(dl_t);
                }
                for t in sub_thread {
                    t.join().expect("failed to join threads")
                }
            });
            thread_pool.push(t);
        }
        for t in thread_pool {
            t.join().unwrap();
        }
        // let mut result = HashMap::new();
        // result.insert(1, "v".to_string())?;
        Ok(())
    }
}

impl New for Sakula {
    fn new() -> Sakula {
        Sakula {
            name: StationName::Sakula,
            host: "http://www.yinghuacd.com".to_string(),
            req: MyRequests::new(),
            movie_name: String::from(""),
        }
    }
}
