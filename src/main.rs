use clap::Parser;
use commands::{RootCommand, SubCommand};

mod commands;

fn main() {
    let app: RootCommand = RootCommand::parse();

    match app.subcmd {
        SubCommand::Server(s) => {
            println!("Starting server at {}", s.host);
        }
    }
}
