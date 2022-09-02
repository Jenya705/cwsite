#![feature(const_trait_impl)]
#![feature(const_mut_refs)]

use actix_web::{App, HttpServer};
use actix_web::web::Data;
use sqlx::{MySql, Pool};
use sqlx::mysql::{MySqlPoolOptions, MySqlQueryResult};

pub mod data;
pub mod api;

pub use crate::data::*;
pub use crate::api::*;

pub type SQLXDatabase = MySql;
pub type SQLXPool = Pool<MySql>;
pub type SQLXQueryResult = MySqlQueryResult;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    env_logger::init();
    let pool = MySqlPoolOptions::new()
        .connect(
            std::env::var("DATABASE_URL")
                .expect("DATABASE_URL should be set in environment")
                .as_str(),
        )
        .await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            // .service(api_configure)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?
    )
}
