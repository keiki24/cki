use crate::models::Article;
use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::Serialize;

#[derive(Serialize)]
pub struct ArticleDecorator {
    pub canonical_path: String,
    pub date: NaiveDate,
    pub japanese_date: String,
    pub published_at_in_rfc2822: String,
    pub id: String,
    pub html_body: String,
    pub title: String,
}

impl From<Article> for ArticleDecorator {
    fn from(article: Article) -> Self {
        Self {
            canonical_path: article.canonical_path(),
            id: article.id(),
            date: article.date,
            japanese_date: format_date_to_japanese(&article.date),
            published_at_in_rfc2822: format_date_to_rfc2822(&article.date),
            html_body: article.html_body,
            title: article.title,
        }
    }
}

fn format_date_to_japanese(date: &NaiveDate) -> String {
    date.format("%Y年%m月%d日").to_string()
}

fn format_date_to_rfc2822(date: &NaiveDate) -> String {
    DateTime::<FixedOffset>::from_utc(date.and_hms(0, 0, 0), FixedOffset::east(60 * 60 * 9))
        .to_rfc2822()
}
