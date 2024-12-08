pub mod error;
pub mod extract;
pub mod recipe;
pub mod user;

use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use axum::{body::Body, http::Request, Router};
use tower::{util::Oneshot, ServiceExt};
use tower_http::cors::CorsLayer;

use crate::core::port;

pub struct App {
    state: AppState,
    router: Router<Arc<AppState>>,
}

pub struct AppState {
    user_service: Arc<dyn port::UserService + Send + Sync>,
    auth_user_service: Arc<dyn port::AuthUserService + Send + Sync>,
    recipe_service: Box<dyn port::RecipeService + Send + Sync>,
}

impl App {
    pub fn new(
        user_service: Arc<dyn port::UserService + Send + Sync>,
        auth_user_service: Arc<dyn port::AuthUserService + Send + Sync>,
        recipe_service: Box<dyn port::RecipeService + Send + Sync>,
    ) -> App {
        Self {
            state: AppState {
                user_service,
                auth_user_service,
                recipe_service,
            },
            router: Router::new()
                .merge(user::build_routes())
                .merge(recipe::build_routes())
                .layer(CorsLayer::permissive()),
        }
    }

    pub fn oneshot(self, req: Request<Body>) -> Oneshot<Router, Request<Body>> {
        self.router.with_state(Arc::new(self.state)).oneshot(req)
    }

    pub fn router(self) -> Router {
        self.router.with_state(Arc::new(self.state))
    }

    pub async fn serve(self, addr: SocketAddr) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .context("error creating tcp listener")?;
        axum::serve(listener, self.router.with_state(Arc::new(self.state)))
            .await
            .context("error running HTTP server")
    }
}
