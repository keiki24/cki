use crate::client::Client;
use crate::path_finder;
use crate::server::run;
use crate::result::Result;
use std::{path::Path, fs};


pub async fn build() -> Result<()> {
    let handle = actix_rt::spawn(async {
        let _ = run().expect("Failed to run server.").await;
    });

    let client = Client::new();
    for path in path_finder::all() {
        println!("{}", path);
        let (bytes, canonical_path) = client.get(&path).await?;
        let output_path_string = format!("output{}", canonical_path);
        let output_path: &Path = output_path_string.as_ref();
        if let Some(parent) = output_path.parent() {
            if !parent.exists() {
                let _ = fs::create_dir_all(parent);
            }
        }
        if !output_path.is_dir() {
            let _ = fs::write(output_path, bytes);
        }
    }

    let _ = handle.abort();
    Ok(())
}