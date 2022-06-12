mod frontmatter;
mod parser;
mod error;
mod handlers;
mod result;
mod models;
mod server;
mod commands;
mod opt;
mod path_finder;
mod client;

use opt::Opt;
use result::Result;
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Serve {} => commands::serve().await,
        Opt::Build {} => commands::build().await,
    }
}