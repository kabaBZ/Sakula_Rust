use scraper::{Html, Selector};

fn main() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Hello World</title>
        </head>
        <body>
            <div class="content">
                <h1>Welcome</h1>
                <p>Hello World</p>
                <ul>
                    <li>Item 1</li>
                    <li>Item 2</li>
                </ul>
            </div>
        </body>
        </html>
    "#;

    let document = Html::parse_document(html);

    // 选择包含“Welcome”文本的h1元素
    let selector = Selector::parse("h1:contains('Welcome')").unwrap();
    let h1_element = document.select(&selector).next().unwrap();
    let h1_text = h1_element.text().collect::<Vec<_>>();
    assert_eq!(h1_text, vec!["Welcome"]);

    // 选择div.content元素下的所有li元素
    let selector = Selector::parse("div.content ul li").unwrap();
    let li_elements = document
        .select(&selector)
        .map(|e| e.text().next().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(li_elements, vec!["Item 1", "Item 2"]);
}
