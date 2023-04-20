use std::net::SocketAddr;

use clap::Parser;
use tracing::{debug};

mod util;

#[derive(Parser, Debug)]
#[command(version, about, after_help = util::AFTER_HELP)]
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
    pub bind_to: SocketAddr,
}

#[derive(clap::Args, Debug)]
pub struct Client {
    /// Start the client
    #[arg(id = "client", long, requires = "client-connect-to")]
    pub enabled: bool,

    /// Specify the server address that the client will connect to
    #[arg(id = "client-connect-to", long, value_name = "ADDR")]
    pub connect_to: Option<SocketAddr>,
}

pub fn parse() -> Cli {
    let cli = Cli::parse();
    debug!(?cli);
    cli
}
