use actix_web::{Result};
use actix_files::NamedFile;
use std::path::PathBuf;

pub async fn html() -> Result<NamedFile> {
    let path: PathBuf = "./views/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}
