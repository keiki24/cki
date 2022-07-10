use actix_web::{HttpServer, App, web::get};
use actix_files::Files;
use crate::{result::Result, handlers::{show_article, show_top_page, show_feed, show_sitemap}};


pub fn run() -> Result<actix_web::dev::Server> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/", get().to(show_top_page))
            .route("/articles/{article_id}", get().to(show_article))
            .route("/feed.xml", get().to(show_feed))
            .route("/sitemap.txt", get().to(show_sitemap))
            .service(Files::new("/", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(server)
}