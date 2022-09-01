use actix_web::{web, get};
use crate::*;

#[get("/find/{id}")]
pub async fn find_player_by_id(path: web::Path<String>, pool: Data<SQLXPool>) -> Result<web::Json<Player>> {
    Ok(web::Json(
        Player::find_by_id(&path.into_inner(), pool.as_ref())
            .await?
            .ok_or_else(|| Error::NotExist)?
    ))
}

#[get("/find/{name}")]
pub async fn find_player_by_name(path: web::Path<String>, pool: Data<SQLXPool>) -> Result<web::Json<Player>> {
    Ok(web::Json(
        Player::find_by_name(&path.into_inner(), pool.as_ref())
            .await?
            .ok_or_else(|| Error::NotExist)?
    ))
}

