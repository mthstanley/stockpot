use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser)]
pub struct RootCommand {
    #[clap(
        value_parser,
        default_value = "127.0.0.1:8080",
        env = "ADDR",
        value_name = "HOST_AND_PORT"
    )]
    pub addr: SocketAddr,

    #[clap(
        value_parser,
        default_value = "localhost",
        env = "DB_HOST",
        value_name = "HOST"
    )]
    pub db_host: String,

    #[clap(
        value_parser,
        default_value = "5432",
        env = "DB_PORT",
        value_name = "PORT"
    )]
    pub db_port: u16,

    #[clap(
        value_parser,
        default_value = "postgres",
        env = "DB_USERNAME",
        value_name = "USERNAME"
    )]
    pub db_username: String,

    #[clap(
        value_parser,
        default_value = "postgres",
        env = "DB_PASSWORD",
        value_name = "PASSWORD"
    )]
    pub db_password: String,

    #[clap(
        value_parser,
        default_value = "stockpot",
        env = "DB_DATABASE",
        value_name = "DATABASE"
    )]
    pub db_database: String,

    #[clap(
        value_parser,
        default_value = "secret",
        env = "JWT_TOKEN_SECRET",
        value_name = "SECRET"
    )]
    pub jwt_token_secret: String,
}
