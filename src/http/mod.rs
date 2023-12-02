use std::net::SocketAddr;

use anyhow::Context;

use axum::routing::get;
use axum::Router;

pub struct App {}

impl App {
    pub fn new() -> App {
        Self {}
    }

    pub async fn serve(self, addr: SocketAddr) -> anyhow::Result<()> {
        axum::Server::bind(&addr)
            .serve(build_routes().into_make_service())
            .await
            .context("eror running HTTP server")
    }
}

pub async fn index() -> &'static str {
    "Hello, World!"
}

fn build_routes() -> Router {
    Router::new().route("/", get(index))
}
