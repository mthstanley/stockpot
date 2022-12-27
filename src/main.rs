use clap::Parser;
use commands::{RootCommand, SubCommand};
use dotenvy::dotenv;

mod commands;

fn main() {
    dotenv().expect(".env file not found");

    let app: RootCommand = RootCommand::parse();

    match app.subcmd {
        SubCommand::Server(s) => {
            println!("Starting server at {}", s.host);
        }
    }
}
