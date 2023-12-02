use anyhow::Ok;
use clap::Parser;
use dotenvy::dotenv;
use log::info;
use stockpot::{
    commands::{RootCommand, SubCommand},
    http,
};

mod commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let app: RootCommand = RootCommand::parse();

    match app.subcmd {
        SubCommand::Server(s) => {
            info!("Starting server at {}", s.addr);
            http::App::new().serve(s.addr).await?;
        }
    }

    Ok(())
}
