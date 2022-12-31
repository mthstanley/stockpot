use anyhow::Context;

use crate::commands::server;
use axum::routing::get;
use axum::Router;

pub async fn index() -> &'static str {
    "Hello, World!"
}

pub async fn server(options: server::RootCommand) -> anyhow::Result<()> {
    axum::Server::bind(&options.addr)
        .serve(build_routes().into_make_service())
        .await
        .context("eror running HTTP server")
}

fn build_routes() -> Router {
    Router::new().route("/", get(index))
}
