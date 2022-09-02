use actix_web::{web, get};
use crate::*;

#[get("/id/{id}")]
pub async fn find_player_by_id(path: web::Path<String>, pool: Data<SQLXPool>) -> Result<web::Json<Player>> {
    Ok(web::Json(
        Player::find_by_id(&path.into_inner(), pool.as_ref())
            .await?
            .ok_or_else(|| Error::NotExist)?
    ))
}

#[get("/name/{name}")]
pub async fn find_player_by_name(path: web::Path<String>, pool: Data<SQLXPool>) -> Result<web::Json<Player>> {
    Ok(web::Json(
        Player::find_by_name(&path.into_inner(), pool.as_ref())
            .await?
            .ok_or_else(|| Error::NotExist)?
    ))
}

pub fn api_configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/player")
            .service(find_player_by_id)
            .service(find_player_by_name)
    );
}