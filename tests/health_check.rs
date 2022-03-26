use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rustapi::app;
use tower::ServiceExt;

#[tokio::test]
async fn health_check_works() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health-check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(body.to_vec().len(), 0);
}
