use axum::{http::StatusCode, response::IntoResponse};

pub async fn feishu_webhook() -> impl IntoResponse {
  StatusCode::OK
}
