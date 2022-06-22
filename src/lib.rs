use std::fmt::Debug;

use axum::{
    extract::{rejection::FormRejection, Form},
    http::StatusCode,
    routing::get,
    routing::post,
    Router,
};

use serde::Deserialize;

pub fn app() -> Router {
    Router::new()
        .route("/health-check", get(health_check))
        .route("/subscribe", post(subscribe))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SubscribeForm {
    name: String,
    email: String,
}

async fn subscribe(input: Result<Form<SubscribeForm>, FormRejection>) -> (StatusCode, String) {
    match input {
        Ok(_) => (StatusCode::OK, "".to_string()),
        Err(rejection) => match rejection {
            FormRejection::FailedToDeserializeQueryString { .. } => {
                (StatusCode::BAD_REQUEST, format! {"{:?}", rejection})
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "".to_string()),
        },
    }
}
