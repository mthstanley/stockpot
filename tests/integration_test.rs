use axum::{
    body::{Body, HttpBody},
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use sqlx::PgPool;
use stockpot::{
    adapters::{http, repositories},
    core::service,
};

#[sqlx::test]
async fn test_get_non_existant_route(pool: PgPool) {
    let app = http::App::new(Box::new(service::DefaultUserService::new(Box::new(
        repositories::PostgresUserRepository::new(pool),
    ))));

    let result = app
        .oneshot(
            Request::builder()
                .uri("/undefined")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::NOT_FOUND);
    assert!(result.into_body().data().await.is_none());
}

#[sqlx::test]
async fn test_get_non_existant_user(pool: PgPool) {
    let app = http::App::new(Box::new(service::DefaultUserService::new(Box::new(
        repositories::PostgresUserRepository::new(pool),
    ))));

    let result = app
        .oneshot(
            Request::builder()
                .uri("/user/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::NOT_FOUND);
    let body = result.into_body().data().await.unwrap().unwrap();
    let json: Value = serde_json::from_slice(&body.to_vec()).unwrap();
    assert_eq!(json, json!({"error": "user with id `1` not found"}));
}

#[sqlx::test(fixtures("user"))]
async fn test_get_existing_user(pool: PgPool) {
    let app = http::App::new(Box::new(service::DefaultUserService::new(Box::new(
        repositories::PostgresUserRepository::new(pool),
    ))));

    let result = app
        .oneshot(
            Request::builder()
                .uri("/user/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::OK);
    let body = result.into_body().data().await.unwrap().unwrap();
    let json: Value = serde_json::from_slice(&body.to_vec()).unwrap();
    assert_eq!(json, json!({"id": 1, "name": "Matt"}));
}

#[sqlx::test(fixtures("user"))]
async fn test_get_invalid_user_id(pool: PgPool) {
    let app = http::App::new(Box::new(service::DefaultUserService::new(Box::new(
        repositories::PostgresUserRepository::new(pool),
    ))));

    let result = app
        .oneshot(
            Request::builder()
                .uri("/user/foo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::BAD_REQUEST);
    let body = result.into_body().data().await.unwrap().unwrap();
    let json: Value = serde_json::from_slice(&body.to_vec()).unwrap();
    assert_eq!(json, json!({"error": "Cannot parse `\"foo\"` to a `i32`"}));
}
