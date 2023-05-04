use crate::routes::{feishu_webhook};
use axum::{
  routing::{post},
  Router, Server,
  middleware::{self, Next},
  response::IntoResponse,
  http::StatusCode,
  response::Response,
  body::{Body, Bytes},
};
use hyper::{Request};
use std::{net::TcpListener};

pub async fn run(listener: TcpListener) -> Result<(), hyper::Error> {
  let app = Router::new()
      .route("/feishu", post(feishu_webhook))
      .layer(middleware::from_fn(print_request_response));

  Server::from_tcp(listener)?
      .serve(app.into_make_service())
      .await
}

async fn print_request_response(
  req: Request<Body>,
  next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
  let (parts, body) = req.into_parts();
  let bytes = buffer_and_print("request", body).await?;

  let slice = bytes.slice(0..bytes.len());

  let body_str = String::from_utf8(slice.to_vec()).unwrap();

  println!("Request body:\n{}", body_str);

  let req = Request::from_parts(parts, Body::from(bytes));

  let res = next.run(req).await;

  let (parts, body) = res.into_parts();
  let bytes = buffer_and_print("response", body).await?;
  let res = Response::from_parts(parts, Body::from(bytes));

  Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
  B: axum::body::HttpBody<Data = Bytes>,
  B::Error: std::fmt::Display,
{
  let bytes = match hyper::body::to_bytes(body).await {
      Ok(bytes) => bytes,
      Err(err) => {
          return Err((
              StatusCode::BAD_REQUEST,
              format!("failed to read {} body: {}", direction, err),
          ));
      }
  };

  Ok(bytes)
}
