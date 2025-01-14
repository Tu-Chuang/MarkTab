use actix_files::NamedFile;
use actix_web::{web, HttpRequest, Result};
use std::path::PathBuf;

pub async fn serve_static(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = NamedFile::open(format!("public/{}", path))?;
    
    Ok(file.use_last_modified(true))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/static")
            .route("/{filename:.*}", web::get().to(serve_static))
    );
} 