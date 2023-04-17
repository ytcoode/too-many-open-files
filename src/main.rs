use tokio::signal;

mod cli;
mod client;
mod server;

#[tokio::main]
async fn main() {
    let cli = cli::parse();

    if cli.server.enabled {
        tokio::spawn(server::start(cli.server.bind_to));
    }

    if cli.client.enabled {}

    signal::ctrl_c().await.expect("signal::ctrl_c");
}
