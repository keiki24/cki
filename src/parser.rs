use crate::frontmatter;
use crate::result::Result;
use pulldown_cmark::{Options, Parser, html};
use scraper::{Selector, Html};
use serde::Deserialize;

pub fn parse(content: &str) -> Result<Data> {
    let result = frontmatter::parse::<Headers>(content)?;
    let html_body = parse_markdown(result.body);
    let title = if result.headers.title == "~" {
        "".to_string()
    } else {
        result.headers.title
    };
    let summary = extract_summary(&html_body);
    Ok(Data {
        html_body,
        summary,
        title,
    })
}

#[derive(Debug)]
pub struct Data {
    pub title: String,
    pub html_body: String,
    pub summary: Option<String>,
}

#[derive(Deserialize)]
struct Headers {
    title: String,
}

fn parse_markdown(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(content, options);
    let mut string = String::new();
    html::push_html(&mut string, parser);
    string
}

fn truncate(str: &str, max_characters_count: usize) -> &str {
    match str.char_indices().nth(max_characters_count) {
        None => str,
        Some((index, _)) => &str[..index],
    }
}

fn extract_summary(html: &str) -> Option<String> {
    let selector = Selector::parse("* > p").unwrap();
    let fragment = Html::parse_fragment(html);
    for element in fragment.select(&selector) {
        let texts: Vec<_> = element.text().collect();
        let inner = texts.join("");
        if !inner.is_empty() {
            let str = inner.split_inclusive("。").next().unwrap();
            let truncated = truncate(str, 140);
            return Some(truncated.to_string());
        }
    }
    None
}