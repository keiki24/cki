use actix_web::{HttpResponse, Responder};
use askama::Template;
use chrono::{Utc, FixedOffset};

use crate::models::{Article, ArticleDecorator};

pub async fn show_feed() -> impl Responder {
    let mut articles: Vec<Article> = Article::all().collect();
    articles.sort_by_key(|article| article.date);
    articles.reverse();
    let size = limit_range(&articles) - 1;
    let articles = articles.drain(..size);
    let body = render(articles).unwrap();
    HttpResponse::Ok()
        .content_type("application/xml; charset=\"utf-8\"")
        .body(body)
}

#[derive(Template)]
#[template(path = "./show_feed.xml")]
struct ShowFeedTemplate<'a> {
    articles: Vec<ArticleDecorator>,
    canonical_url: &'a str,
    current_time_in_rfc2822: &'a str,
    description: &'a str,
    request_base_url: &'a str,
}

fn render(articles: impl Iterator<Item = Article>) -> Result<String, askama::Error> {
    let article_decorators: Vec<ArticleDecorator> =
        articles.map(|article| article.into()).collect();
    let template = ShowFeedTemplate {
        articles: article_decorators,
        current_time_in_rfc2822: &Utc::now().with_timezone(&FixedOffset::east(60 * 60 * 9)).to_rfc2822(),
        canonical_url: "https://keiki24.github.io/feed.xml",
        description: "このウェブサイトでは普段の生活で考えていることを公開しています。",
        request_base_url: "https://keiki24.github.io",
    };
    template.render()
}

fn limit_range(articles: &[Article]) -> usize {
    if articles.len() < 20 {
        articles.len()
    }
    else {
        20
    }

}