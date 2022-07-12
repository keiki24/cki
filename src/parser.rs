use crate::frontmatter;
use crate::result::Result;
use pulldown_cmark::{html, Options, Parser};
use scraper::{Html, Selector};
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
    let image_url = extract_image_url(&html_body);
    Ok(Data {
        html_body,
        image_url,
        summary,
        title,
    })
}

#[derive(Debug)]
pub struct Data {
    pub title: String,
    pub html_body: String,
    pub image_url: Option<String>,
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

fn extract_image_url(html: &str) -> Option<String> {
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("img[src]").unwrap();
    fragment
        .select(&selector)
        .next()
        .map(|element| element.value().attr("src").unwrap().to_string())
}
