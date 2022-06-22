use actix_web::{Responder, HttpResponse};
use askama::Template;
use crate::models::{Article, ArticleDecorator};


pub async fn show_top_page() -> impl Responder {
    let mut articles: Vec<Article> = Article::all().collect();
    articles.sort_by_key(|article| article.date);
    articles.reverse();
    let body = render(articles.into_iter()).expect("Error on rendering template.");
    HttpResponse::Ok()
        .content_type("text/html; charset=\"utf-8\"")
        .body(body)
}

#[derive(Template)]
#[template(path = "./show_top_page.html")]
struct ShowTopPageTemplate<'a> {
    articles: Vec<ArticleDecorator>,
    canonical_url: &'a str,
    image_url: Option<&'a str>,
    og_type: &'a str,
    title: &'a str,
    request_host: &'a str,
}

fn render(articles: impl Iterator<Item = Article>) -> Result<String, askama::Error> {
    let article_decorators: Vec<ArticleDecorator> = articles.map(|article| article.into()).collect();
    let template = ShowTopPageTemplate {
        articles: article_decorators,
        canonical_url: "https://keiki24.github.io/",
        image_url: None, 
        og_type: "website",
        title: "cki.com",
        request_host: "keiki24.github.io",
    };
    template.render()
}