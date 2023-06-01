use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task, time,
};
use tracing::{debug, error, info, instrument};

pub async fn start(addr: SocketAddr) {
    let listener = TcpListener::bind(addr).await.expect("TcpListener::bind");

    info!(
        "Started listening for connections on the address {}.",
        listener.local_addr().expect("listener.local_addr"),
    );

    let counter = Arc::new(AtomicUsize::new(0));

    loop {
        match listener.accept().await {
            Ok((s, _)) => {
                tokio::spawn(server(s, counter.clone()));
                task::yield_now().await;
            }
            Err(e) => {
                error!("An error occurred while calling listener.accept: {}, The number of currently established connnections is {}.",
		       e, counter.load(Ordering::Relaxed));
                time::sleep(Duration::from_secs(10)).await;
            }
        }
    }
}

#[instrument(skip_all, fields(
    peer_addr = %s.peer_addr().unwrap(),
    local_addr = %s.local_addr().unwrap(),
))]
async fn server(mut s: TcpStream, counter: Arc<AtomicUsize>) {
    info!(
        "Successfully accepted a connection. The number of currently established connections is {}.",
        counter.fetch_add(1, Ordering::Relaxed) + 1
    );

    loop {
        let n = match s.read_u8().await {
            Ok(n) => n,
            Err(e) => {
                debug!(
                    "An error occurred while attempting to read a byte from the connection: {}",
                    e
                );
                break;
            }
        };

        debug!("Read a byte: {}", n);

        if let Err(e) = s.write_u8(n).await {
            error!(
                "An error occurred while attempting to write a byte to the connection: {}",
                e
            );
            break;
        }
    }

    info!(
        "Closed the connection. The number of currently established connections is {}.",
        counter.fetch_sub(1, Ordering::Relaxed) - 1
    );
}
