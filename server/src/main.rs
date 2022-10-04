use axum::{Router, Server};
use axum::body::{Bytes, Empty};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::get;

async fn redirect() -> Response<Empty<Bytes>> {
    Response::builder()
      .status(StatusCode::PERMANENT_REDIRECT)
      .header("Location", "https://cdn.discordapp.com/attachments/885709126267732029/1026639328140599316/8db5a3c3dbf13c701c46b7b28a3c685d.jpg")
      .body(Empty::new())
      .unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
      .route("/profile", get(redirect));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}
