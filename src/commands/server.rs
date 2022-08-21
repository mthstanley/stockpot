use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser)]
pub struct RootCommand {
    #[clap(
        value_parser,
        default_value = "127.0.0.1:8080",
        env = "HOST",
        value_name = "HOST_AND_PORT"
    )]
    pub host: SocketAddr,
}
