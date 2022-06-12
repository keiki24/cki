use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Powers cki.")]
pub enum Opt {
    #[structopt(about = "Build statis files.")]
    Build {},

    #[structopt(about = "Run HTTP server.")]
    Serve {},
}