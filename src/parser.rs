use crate::frontmatter;
use crate::result::Result;
use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;

pub fn parse(content: &str) -> Result<Data> {
    let result = frontmatter::parse::<Headers>(content)?;
    let html_body = parse_markdown(result.body);
    let title = if result.headers.title == "~" {
        "".to_string()
    } else {
        result.headers.title
    };
    Ok(Data {
        title,
        html_body,
    })
}

#[derive(Debug)]
pub struct Data {
    pub title: String,
    pub html_body: String,
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