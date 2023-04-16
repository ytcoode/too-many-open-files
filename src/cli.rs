use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
#[group(required = true, args = ["server", "client"])]
pub struct Cli {
    #[command(flatten)]
    pub server: Server,

    #[command(flatten)]
    pub client: Client,
}

#[derive(clap::Args, Debug)]
pub struct Server {
    /// Start the server
    #[arg(id = "server", long)]
    pub enabled: bool,

    /// Specify the address on which the server will listen for connections
    #[arg(
        id = "server-bind-to",
        long,
        value_name = "ADDR",
        default_value = "0.0.0.0:9999"
    )]
    pub addr: SocketAddr,
}

#[derive(clap::Args, Debug)]
pub struct Client {
    /// Start the client
    #[arg(id = "client", long, requires = "client-connect-to")]
    pub enabled: bool,

    /// Specify the server address that the client will connect to
    #[arg(id = "client-connect-to", long, value_name = "ADDR")]
    pub addr: Option<SocketAddr>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
