use actix_web::{self, App, HttpServer};
use draft::{Draft, DraftError};
use serde::Deserialize;
use serde_json::json;
use std::io::Result;
use std::net::SocketAddr;
use tracing::error;
use user::UserId;

mod draft;
mod user;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
