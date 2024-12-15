use std::sync::Arc;

use axum::{
    body::{self, Body},
    http::{request::Builder, Request, StatusCode},
};
use headers::HeaderMapExt;
use serde_json::{json, Value};
use sqlx::PgPool;
use stockpot::{
    adapters::{
        http::{self, recipe::GetRecipe},
        repositories,
    },
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
        String::from("secret"),
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
async fn test_token_authentication_flow(pool: PgPool) {
    let mut app = create_app(pool.clone()).router();

    let mut request = Request::builder().uri("/user/token").method("POST");
    request
        .headers_mut()
        .map(|h| h.typed_insert(headers::Authorization::basic("matt42", "secret")));

    let token_response = app
        .as_service()
        .ready()
        .await
        .unwrap()
        .call(request.body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(token_response.status(), StatusCode::OK);
    let token_body = body::to_bytes(token_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let token_json: Value = serde_json::from_slice(&token_body).unwrap();
    let token = token_json["token"].as_str().unwrap();

    let mut request = Request::builder().uri("/user/auth");
    request
        .headers_mut()
        .map(|h| h.typed_insert(headers::Authorization::bearer(token).unwrap()));

    let result = app
        .as_service()
        .ready()
        .await
        .unwrap()
        .call(request.body(Body::empty()).unwrap())
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
                "yield_units": "grams",
                "ingredients": [
                    {
                        "ingredient": "carrots",
                        "quantity": 200,
                        "units": "grams",
                        "preparation": "diced"
                    },
                    {
                        "ingredient": "butter",
                        "quantity": 200,
                        "units": "grams",
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
    let actual_recipe: GetRecipe = serde_json::from_slice(&body).unwrap();

    let expected_recipe: GetRecipe = serde_json::from_value(json!({
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
        "yield_units": "grams",
        "ingredients": [
            {
                "id": 1,
                "ingredient": "carrots",
                "quantity": 200,
                "units": "grams",
                "preparation": "diced"
            },
            {
                "id": 2,
                "ingredient": "butter",
                "quantity": 200,
                "units": "grams",
                "preparation": "melted"
            }
        ],
        "steps": [
            {
                "id": 1,
                "ordinal": 1,
                "instruction": "Saute the carrots in the butter"
            }
        ]
    }))
    .unwrap();
    assert_eq!(actual_recipe, expected_recipe);

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
    let json: GetRecipe = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, expected_recipe);
}

fn get_authed_request_builder(uri: &str, method: &str) -> Builder {
    let mut request_builder = Request::builder()
        .uri(uri)
        .header("Content-Type", "application/json")
        .method(method);
    request_builder
        .headers_mut()
        .map(|h| h.typed_insert(headers::Authorization::basic("matt42", "secret")));
    request_builder
}

#[sqlx::test(fixtures("user"))]
async fn test_update_recipe_add_new_ingredient(pool: PgPool) {
    let mut app = create_app(pool).router();

    let request = get_authed_request_builder("/recipe", "POST")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "title": "Buttered Carrots",
                "description": "Buttery carrots in a butter sauce",
                "prep_time": 360,
                "cook_time": 400,
                "inactive_time": 8600,
                "yield_quantity": 200,
                "yield_units": "grams",
                "ingredients": [
                    {
                        "ingredient": "carrots",
                        "quantity": 200,
                        "units": "grams",
                        "preparation": "diced"
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
    app.as_service()
        .ready()
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let request = get_authed_request_builder("/recipe/1", "POST")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "id": 1,
                "title": "Buttered Carrots",
                "description": "Buttery carrots in a butter sauce",
                "prep_time": 360,
                "cook_time": 400,
                "inactive_time": 8600,
                "yield_quantity": 200,
                "yield_units": "grams",
                "ingredients": [
                    {
                        "id": 1,
                        "ingredient": "carrots",
                        "quantity": 200,
                        "units": "grams",
                        "preparation": "diced"
                    },
                    {
                        "ingredient": "butter",
                        "quantity": 400,
                        "units": "grams",
                        "preparation": "melted"
                    }
                ],
                "steps": [
                    {
                        "id": 1,
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

    assert_eq!(result.status(), StatusCode::OK);
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: GetRecipe = serde_json::from_slice(&body).unwrap();

    let updated_recipe: GetRecipe = serde_json::from_value(json!({
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
        "yield_units": "grams",
        "ingredients": [
            {
                "id": 1,
                "ingredient": "carrots",
                "quantity": 200,
                "units": "grams",
                "preparation": "diced"
            },
            {
                "id": 2,
                "ingredient": "butter",
                "quantity": 400,
                "units": "grams",
                "preparation": "melted"
            }
        ],
        "steps": [
            {
                "id": 1,
                "ordinal": 1,
                "instruction": "Saute the carrots in the butter"
            }
        ]
    }))
    .unwrap();
    assert_eq!(json, updated_recipe);

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
    let json: GetRecipe = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, updated_recipe);
}

#[sqlx::test(fixtures("user"))]
async fn test_update_recipe_add_new_step(pool: PgPool) {
    let mut app = create_app(pool).router();

    let request = get_authed_request_builder("/recipe", "POST")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "title": "Buttered Carrots",
                "description": "Buttery carrots in a butter sauce",
                "prep_time": 360,
                "cook_time": 400,
                "inactive_time": 8600,
                "yield_quantity": 200,
                "yield_units": "grams",
                "ingredients": [
                    {
                        "ingredient": "carrots",
                        "quantity": 200,
                        "units": "grams",
                        "preparation": "diced"
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
    app.as_service()
        .ready()
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let request = get_authed_request_builder("/recipe/1", "POST")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "id": 1,
                "title": "Buttered Carrots",
                "description": "Buttery carrots in a butter sauce",
                "prep_time": 360,
                "cook_time": 400,
                "inactive_time": 8600,
                "yield_quantity": 200,
                "yield_units": "grams",
                "ingredients": [
                    {
                        "id": 1,
                        "ingredient": "carrots",
                        "quantity": 200,
                        "units": "grams",
                        "preparation": "diced"
                    }
                ],
                "steps": [
                    {
                        "id": 1,
                        "ordinal": 1,
                        "instruction": "Saute the carrots in the butter"
                    },
                    {
                        "ordinal": 2,
                        "instruction": "Eat the carrots"
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

    assert_eq!(result.status(), StatusCode::OK);
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: GetRecipe = serde_json::from_slice(&body).unwrap();

    let updated_recipe: GetRecipe = serde_json::from_value(json!({
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
        "yield_units": "grams",
        "ingredients": [
            {
                "id": 1,
                "ingredient": "carrots",
                "quantity": 200,
                "units": "grams",
                "preparation": "diced"
            }
        ],
        "steps": [
            {
                "id": 1,
                "ordinal": 1,
                "instruction": "Saute the carrots in the butter"
            },
            {
                "id": 2,
                "ordinal": 2,
                "instruction": "Eat the carrots"
            }

        ]
    }))
    .unwrap();
    assert_eq!(json, updated_recipe);

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
    let json: GetRecipe = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, updated_recipe);
}

#[sqlx::test(fixtures("user"))]
async fn test_update_recipe_change_existing_fields(pool: PgPool) {
    let mut app = create_app(pool).router();

    let request = get_authed_request_builder("/recipe", "POST")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "title": "Buttered Carrots",
                "description": "Buttery carrots in a butter sauce",
                "prep_time": 360,
                "cook_time": 400,
                "inactive_time": 8600,
                "yield_quantity": 200,
                "yield_units": "grams",
                "ingredients": [
                    {
                        "ingredient": "carrots",
                        "quantity": 200,
                        "units": "grams",
                        "preparation": "diced"
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
    app.as_service()
        .ready()
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let request = get_authed_request_builder("/recipe/1", "POST")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "id": 1,
                "title": "Boiled Potatoes",
                "description": "Bolied potatoes, that's it.",
                "prep_time": 200,
                "cook_time": 200,
                "inactive_time": 200,
                "yield_quantity": 100,
                "yield_units": "grams",
                "ingredients": [
                    {
                        "id": 1,
                        "ingredient": "potatoes",
                        "quantity": 100,
                        "units": "grams",
                        "preparation": "sliced"
                    }
                ],
                "steps": [
                    {
                        "id": 1,
                        "ordinal": 1,
                        "instruction": "Boil the potatoes"
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

    assert_eq!(result.status(), StatusCode::OK);
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: GetRecipe = serde_json::from_slice(&body).unwrap();

    let updated_recipe: GetRecipe = serde_json::from_value(json!({
        "id": 1,
        "author": {
            "id": 1,
            "name": "Matt"
        },
        "title": "Boiled Potatoes",
        "description": "Bolied potatoes, that's it.",
        "prep_time": 200,
        "cook_time": 200,
        "inactive_time": 200,
        "yield_quantity": 100,
        "yield_units": "grams",
        "ingredients": [
            {
                "id": 1,
                "ingredient": "potatoes",
                "quantity": 100,
                "units": "grams",
                "preparation": "sliced"
            }
        ],
        "steps": [
            {
                "id": 1,
                "ordinal": 1,
                "instruction": "Boil the potatoes"
            }
        ]
    }))
    .unwrap();
    assert_eq!(json, updated_recipe);

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
    let json: GetRecipe = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, updated_recipe);
}

#[sqlx::test(fixtures("user"))]
async fn test_update_recipe_remove_ingredient_step(pool: PgPool) {
    let mut app = create_app(pool).router();

    let request = get_authed_request_builder("/recipe", "POST")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "title": "Buttered Carrots",
                "description": "Buttery carrots in a butter sauce",
                "prep_time": 360,
                "cook_time": 400,
                "inactive_time": 8600,
                "yield_quantity": 200,
                "yield_units": "grams",
                "ingredients": [
                    {
                        "ingredient": "carrots",
                        "quantity": 200,
                        "units": "grams",
                        "preparation": "diced"
                    },
                    {
                        "ingredient": "butter",
                        "quantity": 200,
                        "units": "grams",
                        "preparation": "melted"
                    }
                ],
                "steps": [
                    {
                        "ordinal": 1,
                        "instruction": "Saute the carrots in the butter"
                    },
                    {
                        "ordinal": 2,
                        "instruction": "Eat the carrots"
                    }
                ]
            }))
            .unwrap(),
        ))
        .unwrap();
    app.as_service()
        .ready()
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let request = get_authed_request_builder("/recipe/1", "POST")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "id": 1,
                "title": "Buttered Carrots",
                "description": "Buttery carrots in a butter sauce",
                "prep_time": 360,
                "cook_time": 400,
                "inactive_time": 8600,
                "yield_quantity": 200,
                "yield_units": "grams",
                "ingredients": [
                    {
                        "id": 1,
                        "ingredient": "carrots",
                        "quantity": 200,
                        "units": "grams",
                        "preparation": "diced"
                    }
                ],
                "steps": [
                    {
                        "id": 1,
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

    assert_eq!(result.status(), StatusCode::OK);
    let body = body::to_bytes(result.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: GetRecipe = serde_json::from_slice(&body).unwrap();

    let updated_recipe: GetRecipe = serde_json::from_value(json!({
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
        "yield_units": "grams",
        "ingredients": [
            {
                "id": 1,
                "ingredient": "carrots",
                "quantity": 200,
                "units": "grams",
                "preparation": "diced"
            }
        ],
        "steps": [
            {
                "id": 1,
                "ordinal": 1,
                "instruction": "Saute the carrots in the butter"
            }
        ]
    }))
    .unwrap();
    assert_eq!(json, updated_recipe);

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
    let json: GetRecipe = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, updated_recipe);
}

#[sqlx::test(fixtures("user"))]
async fn test_delete_recipe(pool: PgPool) {
    let mut app = create_app(pool).router();

    let request = get_authed_request_builder("/recipe", "POST")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "title": "Buttered Carrots",
                "description": "Buttery carrots in a butter sauce",
                "prep_time": 360,
                "cook_time": 400,
                "inactive_time": 8600,
                "yield_quantity": 200,
                "yield_units": "grams",
                "ingredients": [
                    {
                        "ingredient": "carrots",
                        "quantity": 200,
                        "units": "grams",
                        "preparation": "diced"
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
    app.as_service()
        .ready()
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let created_recipe: GetRecipe = serde_json::from_value(json!({
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
        "yield_units": "grams",
        "ingredients": [
            {
                "id": 1,
                "ingredient": "carrots",
                "quantity": 200,
                "units": "grams",
                "preparation": "diced"
            }
        ],
        "steps": [
            {
                "id": 1,
                "ordinal": 1,
                "instruction": "Saute the carrots in the butter"
            }
        ]
    }))
    .unwrap();

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
    let json: GetRecipe = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, created_recipe);

    assert_eq!(
        app.as_service()
            .ready()
            .await
            .unwrap()
            .call(
                get_authed_request_builder("/recipe/1", "DELETE")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap()
            .status(),
        StatusCode::OK
    );

    assert_eq!(
        app.as_service()
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
            .unwrap()
            .status(),
        StatusCode::NOT_FOUND
    );
}
