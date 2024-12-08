use clap::{AppSettings, Parser};

pub mod server;

#[derive(Parser)]
#[clap(
    name = "stockpot",
    author = "Matt Stanley <stanley.t.matthew@gmail.com>",
    about = "Commands for running the Stockpot App API"
)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct RootCommand {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
    Server(server::RootCommand),
}
