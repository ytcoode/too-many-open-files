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
    net::TcpStream,
    task, time,
};
use tracing::{debug, error, info, instrument};

pub async fn start(addr: SocketAddr) {
    let counter = Arc::new(AtomicUsize::new(0));

    loop {
        match TcpStream::connect(addr).await {
            Ok(s) => {
                tokio::spawn(client(s, counter.clone()));
                task::yield_now().await;
            }
            Err(e) => {
                error!(
                    "An error occurred while attempting to connect to {}: {}. The number of currently established connections is {}",
                    addr, e, counter.load(Ordering::Relaxed)
                );
                time::sleep(Duration::from_secs(10)).await;
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

    let mut n = 0u8;

    loop {
        time::sleep(Duration::from_secs(60 * 10)).await;

        if let Err(e) = s.write_u8(n).await {
            debug!(
                "An error occurred while attempting to write a byte to the connection: {}",
                e
            );
            break;
        }

        match s.read_u8().await {
            Ok(m) => assert_eq!(m, n),
            Err(e) => {
                debug!(
                    "An error occurred while attempting to read a byte from the connection: {}",
                    e
                );
                break;
            }
        }

        n = n.wrapping_add(1);
    }

    info!(
        "Closed the connection. The number of currently established connections is {}.",
        counter.fetch_sub(1, Ordering::Relaxed) - 1
    );
}
