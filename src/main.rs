use anyhow::Ok;
use clap::Parser;
use dotenvy::dotenv;
use log::info;
use stockpot::{
    adapters::{http, repositories},
    commands::{RootCommand, SubCommand},
    core::service,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let app: RootCommand = RootCommand::parse();

    match app.subcmd {
        SubCommand::Server(s) => {
            info!("Starting server at {}", s.addr);
            let connect_options = sqlx::postgres::PgConnectOptions::new()
                .host(&s.db_host)
                .port(s.db_port)
                .username(&s.db_username)
                .password(&s.db_password)
                .database(&s.db_database);
            info!(
                "Attempting db connection at postgres://{}:{}@{}:{}/{}",
                s.db_username,
                "*".repeat(s.db_password.len()),
                s.db_host,
                s.db_port,
                s.db_database
            );
            let pool = sqlx::postgres::PgPoolOptions::new()
                .connect_with(connect_options)
                .await?;
            let user_service = Box::new(service::DefaultUserService::new(Box::new(
                repositories::PostgresUserRepository::new(pool),
            )));
            http::App::new(user_service).serve(s.addr).await?;
        }
    }

    Ok(())
}
