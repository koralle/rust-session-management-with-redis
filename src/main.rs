use serde::Deserialize;
use std::net::SocketAddr;
use user::UserId;
use uuid::Uuid;

use axum::{routing::get, Router};

mod user;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Deserialize)]
struct FetchDraft {
    user_id: UserId,
}
