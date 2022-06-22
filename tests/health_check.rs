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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/subscribe")
                .method("POST")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from(
                    "name=le%20guin&email=ursula_le_guin%40gmail.com",
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    // .expect("Failed to execute request.");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let tests = vec![
        ("name=le%20guin", "missing email field"),
        ("email=ursula_le_guin%40gmail.com", "missing name field"),
        ("", "missing all fields"),
    ];

    for (data, message) in tests {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/subscribe")
                    .method("POST")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(Body::from(data))
                    .unwrap(),
            )
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "did not get expected status code when payload was {}",
            message
        );
    }
}
