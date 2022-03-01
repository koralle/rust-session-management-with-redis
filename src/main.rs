use actix_web::{
    self, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use draft::{Draft, DraftError};
use serde::Deserialize;
use serde_json::json;
use std::error::Error;
use std::net::SocketAddr;
use tracing::error;
use user::UserId;

mod draft;
mod user;

#[actix_web::main]
async fn main() -> actix_web::Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "debug");
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

pub async fn hello(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello, world")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[actix_rt::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default().to_http_request();
        let response = hello(req).await;

        assert_eq!(response.status(), http::StatusCode::OK)
    }
}
