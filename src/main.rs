use tokio::signal;

mod cli;
mod client;
mod server;
mod tracing;

#[tokio::main]
async fn main() {
    tracing::init();

    let cli = cli::parse();

    if cli.server.enabled {
        tokio::spawn(server::start(cli.server.bind_to));
    }

    if cli.client.enabled {
        tokio::spawn(client::start(cli.client.connect_to.unwrap()));
    }

    signal::ctrl_c().await.expect("signal::ctrl_c");
}
