use std::{thread, time::Duration};

use tokio::{signal, time};
use tracing::info;

mod cli;
mod client;
mod init;
mod server;

fn main() {
    init::init();

    let cli = cli::parse();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        if cli.server.enabled {
            server::start(cli.server.bind_to, cli.r#async);
        }

        if cli.client.enabled {
            tokio::spawn(client::start(cli.client.connect_to.unwrap()));
        }

        info!("Press Ctrl-C to shut down");
        signal::ctrl_c().await.expect("signal::ctrl_c");
    });

    rt.shutdown_background();
    info!("Shutdown completed successfully");
}
