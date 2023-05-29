use std::{
    io::ErrorKind,
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use tokio::{io::AsyncReadExt, net::TcpStream, time};
use tracing::{debug, error, info, instrument};

pub async fn start(addr: SocketAddr) {
    let counter = Arc::new(AtomicUsize::new(0));

    loop {
        match TcpStream::connect(addr).await {
            Ok(s) => {
                tokio::spawn(client(s, counter.clone()));
            }

            Err(e) => {
                error!(
                    "An error occurred while attempting to connect to {}: {}. The number of currently established connections is {}",
                    addr, e, counter.load(Ordering::Relaxed)
                );
                time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

#[instrument(skip_all, fields(
    local_addr = %s.local_addr().unwrap(),
    peer_addr = %s.peer_addr().unwrap(),
))]
async fn client(mut s: TcpStream, counter: Arc<AtomicUsize>) {
    info!(
        "Successfully made a connection. The number of currently established connections is {}.",
        counter.fetch_add(1, Ordering::Relaxed) + 1
    );

    match s.read_u8().await {
        Ok(_) => unreachable!(),
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
            debug!("Connection closed by remote peer")
        }
        Err(e) => error!(
            "An error occurred while attempting to read a byte from the connection: {}",
            e
        ),
    }

    info!(
        "Closed the connection. The number of currently established connections is {}.",
        counter.fetch_sub(1, Ordering::Relaxed) - 1
    );
}
