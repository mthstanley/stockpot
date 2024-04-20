use axum::{
    body::{self, Body},
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use sqlx::PgPool;
use stockpot::{
    adapters::{
        http::{self},
        repositories,
    },
    core::service,
};
use tower::{Service, ServiceExt};

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
    assert_eq!(
        body::to_bytes(result.into_body(), usize::MAX)
            .await
            .unwrap()
            .len(),
        0
    );
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
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
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
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
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
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, json!({"error": "Cannot parse `\"foo\"` to a `i32`"}));
}

#[sqlx::test]
async fn test_create_user_success(pool: PgPool) {
    let mut app = http::App::new(Box::new(service::DefaultUserService::new(Box::new(
        repositories::PostgresUserRepository::new(pool),
    ))))
    .router();

    let result = app
        .as_service()
        .ready()
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/user")
                .header("Content-Type", "application/json")
                .method("POST")
                .body(Body::from(
                    serde_json::to_vec(&json!({ "name": "Tom" })).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::CREATED);
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, json!({"id": 1, "name": "Tom"}));

    let result = app
        .as_service()
        .ready()
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/user/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::OK);
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, json!({"id": 1, "name": "Tom"}));
}
