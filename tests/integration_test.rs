use std::sync::Arc;

use axum::{
    body::{self, Body},
    http::{Request, StatusCode},
};
use headers::HeaderMapExt;
use serde_json::{json, Value};
use sqlx::PgPool;
use stockpot::{
    adapters::{http, repositories},
    core::service,
};
use tower::{Service, ServiceExt};

fn create_app(pool: PgPool) -> http::App {
    let user_service = Arc::new(service::DefaultUserService::new(Box::new(
        repositories::PostgresUserRepository::new(pool.clone()),
    )));
    let auth_service = Arc::new(service::DefaultAuthUserService::new(
        Box::new(repositories::PostgresAuthUserRepository::new(pool.clone())),
        user_service.clone(),
    ));
    let recipe_service = Box::new(service::DefaultRecipeService::new(Box::new(
        repositories::PostgresRecipeRepository::new(pool),
    )));
    http::App::new(user_service, auth_service, recipe_service)
}

#[sqlx::test]
async fn test_get_non_existant_route(pool: PgPool) {
    let app = create_app(pool.clone());

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
    let app = create_app(pool.clone());

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
    let app = create_app(pool.clone());

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
    let app = create_app(pool.clone());

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
    let mut app = create_app(pool.clone()).router();

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
                    serde_json::to_vec(
                        &json!({ "name": "Tom", "username": "tom333", "password": "secret" }),
                    )
                    .unwrap(),
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

#[sqlx::test(fixtures("user"))]
async fn test_error_missing_credentials(pool: PgPool) {
    let app = create_app(pool.clone());

    let result = app
        .oneshot(
            Request::builder()
                .uri("/user/auth")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::UNAUTHORIZED);
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, json!({"error": "Invalid credentials"}));
}

#[sqlx::test(fixtures("user"))]
async fn test_successful_authentication(pool: PgPool) {
    let app = create_app(pool.clone());

    let mut request = Request::builder().uri("/user/auth");
    request
        .headers_mut()
        .map(|h| h.typed_insert(headers::Authorization::basic("matt42", "secret")));

    let result = app
        .oneshot(request.body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::OK);
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        json,
        json!({"message": "Successful authentication for Matt"})
    );
}

#[sqlx::test(fixtures("user"))]
async fn test_create_recipe(pool: PgPool) {
    let mut app = create_app(pool).router();

    let mut request_builder = Request::builder()
        .uri("/recipe")
        .header("Content-Type", "application/json")
        .method("POST");
    request_builder
        .headers_mut()
        .map(|h| h.typed_insert(headers::Authorization::basic("matt42", "secret")));

    let request = request_builder
        .body(Body::from(
            serde_json::to_vec(&json!({
                "title": "Buttered Carrots",
                "description": "Buttery carrots in a butter sauce",
                "prep_time": 360,
                "cook_time": 400,
                "inactive_time": 8600,
                "yield_quantity": 200,
                "yield_units": {
                    "name": "grams"
                },
                "ingredients": [
                    {
                        "ingredient": {
                            "name": "carrots"
                        },
                        "quantity": 200,
                        "units": {
                            "name": "grams"
                        },
                        "preparation": "diced"
                    },
                    {
                        "ingredient": {
                            "name": "butter"
                        },
                        "quantity": 200,
                        "units": {
                            "name": "grams"
                        },
                        "preparation": "melted"
                    }
                ],
                "steps": [
                    {
                        "ordinal": 1,
                        "instruction": "Saute the carrots in the butter"
                    }
                ]
            }))
            .unwrap(),
        ))
        .unwrap();
    let result = app
        .as_service()
        .ready()
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(result.status(), StatusCode::CREATED);
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let created_recipe = json!({
        "id": 1,
        "author": {
            "id": 1,
            "name": "Matt"
        },
        "title": "Buttered Carrots",
        "description": "Buttery carrots in a butter sauce",
        "prep_time": 360,
        "cook_time": 400,
        "inactive_time": 8600,
        "yield_quantity": 200,
        "yield_units": {
            "id": 1,
            "name": "grams"
        },
        "ingredients": [
            {
                "id": 1,
                "recipe_id": 1,
                "ingredient": {
                    "id": 1,
                    "name": "carrots"
                },
                "quantity": 200,
                "units": {
                    "id": 1,
                    "name": "grams"
                },
                "preparation": "diced"
            },
            {
                "id": 2,
                "recipe_id": 1,
                "ingredient": {
                    "id": 2,
                    "name": "butter"
                },
                "quantity": 200,
                "units": {
                    "id": 1,
                    "name": "grams"
                },
                "preparation": "melted"
            }
        ],
        "steps": [
            {
                "id": 1,
                "recipe_id": 1,
                "ordinal": 1,
                "instruction": "Saute the carrots in the butter"
            }
        ]
    });
    assert_eq!(json, created_recipe);

    let result = app
        .as_service()
        .ready()
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/recipe/1")
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
    assert_eq!(json, created_recipe);
}
