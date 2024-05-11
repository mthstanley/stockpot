pub mod error;
pub mod extract;
pub mod user;

use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use axum::{body::Body, http::Request, Router};
use tower::{util::Oneshot, ServiceExt};

use crate::core::{domain, port};

pub struct App {
    state: AppState,
    router: Router<Arc<AppState>>,
}

pub struct AppState {
    user_service: Arc<dyn port::UserService + Send + Sync>,
    auth_user_service: Arc<
        dyn port::AuthUserService<domain::UserCredentials, domain::UserCredentials> + Send + Sync,
    >,
}

impl App {
    pub fn new(
        user_service: Arc<dyn port::UserService + Send + Sync>,
        auth_user_service: Arc<
            dyn port::AuthUserService<domain::UserCredentials, domain::UserCredentials>
                + Send
                + Sync,
        >,
    ) -> App {
        Self {
            state: AppState {
                user_service,
                auth_user_service,
            },
            router: user::build_routes(),
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
