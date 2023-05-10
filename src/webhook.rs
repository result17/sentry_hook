use std::{collections::{HashMap}};

#[derive(serde::Serialize, serde::Deserialize)]
struct WebhookStacktraceFrame {
  function: String,
  module: String,
  filename: String,
  abs_path: String,
  lineno: u32,
  pre_context: Vec<String>,
  conext_line: String,
  post_context: Vec<String>,
  in_app: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct WebhookStacktrace {
  frames: Vec<WebhookStacktraceFrame>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct WebhookClientOs {
  name: String,
  version: String,
  #[serde(rename = "type")]
  _type: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct WebhookContextsBrowser {
  name: String,
  version: String,
  #[serde(rename = "type")]
  _type: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct WebhookContexts {
  browser: WebhookContextsBrowser,

}

#[derive(serde::Serialize, serde::Deserialize)]
struct WebhookUserGeo {
  country_code: String,
  city: String,
  region: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct WebhookUser {
  id: String,
  email: String,
  ip_address: String,
  username: String,
  name: String,
  geo: WebhookUserGeo,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct WebhookEvent {
  event_id: String,
  level: String,
  version: String,
  #[serde(rename = "type")]
  _type: String,
  #[serde(skip_serializing, skip_deserializing)]
  logentry: HashMap<String, Option<String>>,
  logger: String,
  #[serde(skip_serializing, skip_deserializing)]
  modules: HashMap<String, String>,
  platform: String,
  timestamp: f32,
  received: f32,
  environment: String,
  user: WebhookUser,
  #[serde(skip_serializing, skip_deserializing)]
  request: HashMap<String, String>,

}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebhookRequest {
  id: String,
  project: String,
  project_name: String,
  logger: Option<String>,
  level: String,
  curlprit: String,
  message: String,
  url: String,
  #[serde(skip_serializing, skip_deserializing)]
  triggering_rules: String,
  contexts: WebhookContexts,
  stacktrace: WebhookStacktrace,
  tags: Vec<Vec<String>>
}
