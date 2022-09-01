use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Player with this id or name already exists")]
    PlayerAlreadyExists,
    #[error("Entity is not exist")]
    NotExist,
    #[error("Sqlx error: {0}")]
    SQLX(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::PlayerAlreadyExists => StatusCode::BAD_REQUEST,
            Self::NotExist => StatusCode::NOT_FOUND,
            Self::SQLX(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status_code = self.status_code();
        let mut response = HttpResponse::new(status_code);
        if !status_code.is_server_error() {
            response = response.set_body(BoxBody::new(
                format!("{{\"error\":\"{}\"}}", self)
            ));
        }
        response
    }
}

impl Player {
    pub async fn new(id: String, name: String, pool: &SQLXPool) -> Result<Self> {
        match Player::find_by_id(&id, pool).await?.is_some() ||
            Player::find_by_name(&name, pool).await?.is_some() {
            true => Err(Error::PlayerAlreadyExists),
            false => Ok(Self { id, name }),
        }
    }

    pub async fn find_by_id(id: &String, pool: &SQLXPool) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Player,
            r#"SELECT * FROM players WHERE id = ?"#,
            id
        ).fetch_optional(pool).await
    }

    pub async fn find_by_name(name: &String, pool: &SQLXPool) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Player,
            r#"SELECT * FROM players WHERE name = ?"#,
            name
        ).fetch_optional(pool).await
    }

    pub async fn update(&self, pool: &SQLXPool) -> sqlx::Result<SQLXQueryResult> {
        sqlx::query!(
            r#"UPDATE players
            SET name = ?
            WHERE id = ?"#,
            self.name, self.id
        ).execute(pool).await
    }
}