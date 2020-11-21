use std::path::PathBuf;
use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, Error};


#[get("/")]
pub async fn main() -> Result<fs::NamedFile, Error> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(fs::NamedFile::open(path)?)
}