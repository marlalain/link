use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use axum::{Router, Server};
use axum::body::{Bytes, Empty};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::get;
use axum::extract::{Path};

fn links(q: &str) -> &str {
    std::collections::HashMap::from([
        ("twitter", "https://twitter.com/alexsyntaxtree"),
        ("github", "https://github.com/minsk-dev")
    ]).get(q).unwrap_or(&"https://github.com/minsk-dev")
}

async fn redirect(Path(id): Path<String>) -> Response<Empty<Bytes>> {
    Response::builder()
      .status(StatusCode::PERMANENT_REDIRECT)
      .header("location", links(&id))
      .body(Empty::new())
      .unwrap()
}

async fn default() -> Response<Empty<Bytes>> {
    Response::builder()
      .status(StatusCode::PERMANENT_REDIRECT)
      .header("location", "https://twitter.com/alexsyntaxtree")
      .body(Empty::new())
      .unwrap()
}

async fn profile() -> Response<Empty<Bytes>> {
    Response::builder()
      .status(StatusCode::PERMANENT_REDIRECT)
      .header("location", "https://cdn.discordapp.com/attachments/885709126267732029/1026639328140599316/8db5a3c3dbf13c701c46b7b28a3c685d.jpg")
      .body(Empty::new())
      .unwrap()
}

async fn discord(Path((id, id2, img)): Path<(String, String, String)>) -> Response<Empty<Bytes>> {
    Response::builder()
      .status(StatusCode::PERMANENT_REDIRECT)
      .header("location", format!("https://cdn.discordapp.com/attachments/{}/{}/{}", id, id2, img))
      .body(Empty::new())
      .unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
      .route("/l/:id", get(redirect))
      .route("/", get(github))
      .route("/d/:id/:id2/:img", get(discord))
      .route("/profile", get(profile));
    let ip = env::var("IP").unwrap_or("0.0.0.0".parse().unwrap());
    let port = env::var("PORT").unwrap_or("80".parse().unwrap());
    let addr = format!("{}:{}", ip, port);

    Server::bind(&SocketAddr::from_str(&addr).unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}
