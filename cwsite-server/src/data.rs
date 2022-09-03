use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Player with id {id} or name {name} already exists")]
    PlayerAlreadyExists {
        id: String,
        name: String,
    },
    #[error("Entity is not exist")]
    NotExist,
    #[error("Sqlx error: {0}")]
    SQLX(#[from] sqlx::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Unknown error: {0}")]
    Any(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MainRole {
    Default,
    Premium,
    TestModerator,
    Moderator,
    MainModerator,
    Developer,
    Admin = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum SecondaryRole {}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Debug)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub discord_id: u64,
    pub role: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Debug)]
pub struct AuthenticationToken {
    pub discord_id: u64,
    pub token: String,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::PlayerAlreadyExists { .. } => StatusCode::BAD_REQUEST,
            Self::NotExist => StatusCode::NOT_FOUND,
            Self::SQLX(_) | Self::Reqwest(_) => StatusCode::INTERNAL_SERVER_ERROR,
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

impl const PartialEq for MainRole {
    fn eq(&self, other: &Self) -> bool {
        *self as u8 == *other as u8
    }
}

impl const From<u8> for MainRole {
    fn from(value: u8) -> Self {
        match value & 7 {
            1 => Self::Premium,
            2 => Self::TestModerator,
            3 => Self::Moderator,
            4 => Self::MainModerator,
            5 => Self::Developer,
            7 => Self::Admin,
            _ => Self::Default,
        }
    }
}

impl MainRole {
    pub const fn is_permitted(self, role: Self) -> bool {
        role == self || (self != Self::Developer && self as u8 >= role as u8)
    }

    pub const fn is_moderator(self) -> bool {
        matches!(self, Self::TestModerator | Self::Moderator | Self::MainModerator | Self::Admin)
    }

    pub const fn is_developer(self) -> bool {
        matches!(self, Self::Developer | Self::Admin)
    }

    pub const fn is_premium(self) -> bool {
        !matches!(self, Self::Default)
    }
}

impl Player {
    pub const fn get_main_role(&self) -> MainRole {
        MainRole::from(self.role as u8)
    }

    pub const fn set_main_role(&mut self, role: MainRole) {
        self.role = self.role & !7 | (role as u64)
    }

    pub const fn new(id: String, name: String, discord_id: u64) -> Self {
        Self { id, name, discord_id, role: 0 }
    }

    pub async fn find_by_token(token: &String, pool: &SQLXPool) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Player,
            r#"SELECT players.* FROM players
            INNER JOIN authentication_tokens
            ON authentication_tokens.discord_id = players.discord_id AND authentication_tokens.token = ?"#,
            token
        ).fetch_optional(pool).await
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

    pub async fn find_by_discord_id(id: &u64, pool: &SQLXPool) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(
            Player,
            r#"SELECT * FROM players WHERE discord_id = ?"#,
            id
        ).fetch_optional(pool).await
    }

    pub async fn update(&self, pool: &SQLXPool) -> sqlx::Result<SQLXQueryResult> {
        sqlx::query!(
            r#"UPDATE players SET name = ?, discord_id = ?, role = ? WHERE id = ?"#,
            self.name, self.discord_id, self.role, self.id
        ).execute(pool).await
    }

    pub async fn insert(&self, pool: &SQLXPool) -> sqlx::Result<SQLXQueryResult> {
        sqlx::query!(
            r#"INSERT INTO players (id, name, discord_id, role) VALUES (?, ?, ?, ?)"#,
            self.id, self.name, self.discord_id, self.role
        ).execute(pool).await
    }
}

impl AuthenticationToken {
    pub fn new(discord_id: u64) -> Self {
        let mut token = String::with_capacity(128);
        token.push_str(format!("{:X}", discord_id).as_str());
        let mut bytes = [0u8; 128 / 2 - std::mem::size_of::<u64>()];
        getrandom::getrandom(&mut bytes).unwrap();
        for byte in bytes {
            token.push(Self::choose_char(byte & 63));
            token.push(Self::choose_char(byte >> 6));
        }
        Self { discord_id, token }
    }

    const fn choose_char(value: u8) -> char {
        (match value {
            0..=25 => value + ('a' as u8),
            26..=51 => (value - 26) + ('A' as u8),
            52..=61 => (value - 52) + ('0' as u8),
            62 => '[' as u8,
            _ => ']' as u8
        }) as char
    }

    pub async fn find_by_token(token: &String, pool: &SQLXPool) -> sqlx::Result<Option<AuthenticationToken>> {
        sqlx::query_as!(
            AuthenticationToken,
            r#"SELECT * FROM authentication_tokens WHERE token = ?"#,
            token
        ).fetch_optional(pool).await
    }

    pub async fn insert(&self, pool: &SQLXPool) -> sqlx::Result<SQLXQueryResult> {
        sqlx::query!(
            r#"INSERT INTO authentication_tokens (discord_id, token) VALUES (?, ?)"#,
            self.discord_id, self.token
        ).execute(pool).await
    }

    pub async fn delete_by_id(id: u64, pool: &SQLXPool) -> sqlx::Result<SQLXQueryResult> {
        sqlx::query!(
            r#"DELETE FROM authentication_tokens WHERE discord_id = ?"#,
            id
        ).execute(pool).await
    }
}