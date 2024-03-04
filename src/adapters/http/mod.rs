pub mod error;
pub mod extract;
pub mod user;

use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use axum::{body::Body, http::Request, Router};
use tower::{util::Oneshot, ServiceExt};

use crate::core::port;

pub struct App {
    state: AppState,
    router: Router<Arc<AppState>>,
}

pub struct AppState {
    user_service: Box<dyn port::UserService + Send + Sync>,
}

impl App {
    pub fn new(user_service: Box<dyn port::UserService + Send + Sync>) -> App {
        Self {
            state: AppState { user_service },
            router: user::build_routes(),
        }
    }

    pub fn oneshot(self, req: Request<Body>) -> Oneshot<Router, Request<Body>> {
        self.router.with_state(Arc::new(self.state)).oneshot(req)
    }

    pub async fn serve(self, addr: SocketAddr) -> anyhow::Result<()> {
        axum::Server::bind(&addr)
            .serve(
                self.router
                    .with_state(Arc::new(self.state))
                    .into_make_service(),
            )
            .await
            .context("error running HTTP server")
    }
}
