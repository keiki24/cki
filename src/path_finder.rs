use globwalk::glob;
use crate::models::Article;

pub fn all() -> impl Iterator<Item = String> {
    article_paths().chain(other_paths()).chain(asset_paths()) 
}

pub fn article_paths() -> impl Iterator<Item = String> {
    Article::all().map(|article| format!("/articles/{}", article.id())) 
}

fn asset_paths() -> impl Iterator<Item = String> {
    glob("static/**/*")
        .unwrap()
        .flatten()
        .map(|entry| entry.path().to_owned())
        .filter(|path| path.is_file())
        .map(|path| path.to_str().unwrap()[8..].to_string())
}

fn other_paths() -> impl Iterator<Item = String> {
    ["/", "/feed.xml", "/sitemap.txt"]
    .iter()
    .map(|&element| element.into())
}