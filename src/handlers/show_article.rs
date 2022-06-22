use actix_web::{web::Path, Responder, HttpResponse};
use askama::Template;
use crate::models::{ArticleDecorator, Article};

pub async fn show_article(article_id: Path<String>) -> impl Responder {
    let article = Article::find(&article_id).unwrap();
    let body = render(article).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=\"utf-8\"")
        .body(body)
}

#[derive(Template)]
#[template(path = "./show_article.html")]
struct ShowArticleTemplate<'a> {
    article: &'a ArticleDecorator,
    canonical_url: &'a str,
    image_url: Option<&'a str>,
    og_type: &'a str,
    title: &'a str,    
    request_host: &'a str,
}

fn render(article: Article) -> Result<String, askama::Error> {
    let article: ArticleDecorator = article.into();
    let canonical_url = format!("https://keiki24.github.io{}", article.canonical_path);
    let template = ShowArticleTemplate {
        article: &article,
        canonical_url: &canonical_url,
        image_url: None,
        og_type: "article",
        title: &article.title,
        request_host: "keiki24.github.io",
    };
    template.render()
}

#[cfg(test)]
mod tests {
    use super::render;
    use crate::models::Article;

    #[test]
    fn render_works() {
        let article = Article::find("2000-01-01-example").unwrap();
        let result = render(article);
        assert!(result.is_ok());
    }
}