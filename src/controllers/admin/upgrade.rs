use actix_web::{web, HttpResponse};
use crate::{error::AppError, services::upgrade::UpgradeService};

pub async fn check_update() -> Result<HttpResponse, AppError> {
    let update = UpgradeService::check_update().await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "code": 1,
        "msg": "success",
        "data": update
    })))
}

pub async fn do_upgrade() -> Result<HttpResponse, AppError> {
    if let Some(version) = UpgradeService::check_update().await? {
        UpgradeService::download_update(&version).await?;
        
        Ok(HttpResponse::Ok().json(json!({
            "code": 1,
            "msg": "System upgraded successfully",
            "data": version
        })))
    } else {
        Ok(HttpResponse::Ok().json(json!({
            "code": 0,
            "msg": "No updates available",
            "data": null::<()>
        })))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/upgrade")
            .route("/check", web::get().to(check_update))
            .route("/do", web::post().to(do_upgrade))
    );
} 