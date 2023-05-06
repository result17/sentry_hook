use axum::{ response::IntoResponse};

pub async fn hello_world() -> impl IntoResponse {
  "Hello world"
}
