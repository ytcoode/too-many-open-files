use std::{io::ErrorKind, net::SocketAddr, time::Duration};

use tokio::{io::AsyncReadExt, net::TcpStream, time};
use tracing::{debug, error, info, instrument};

pub async fn start(addr: SocketAddr) {
    let mut counter = 0;

    loop {
        match TcpStream::connect(addr).await {
            Err(e) => {
                error!(
                    "An error occurred while attempting to connect to {}: {}.",
                    addr, e,
                );
                break;
            }
            Ok(s) => {
                tokio::spawn(process_socket(s));
                counter += 1;
                info!("Successfully made a connection to {}. The number of currently established connections is {}.", addr, counter);
            }
        }

        // time::sleep(Duration::from_millis(200)).await; // TODO connect interval
    }
}

#[instrument(skip_all, fields(
    local_addr = %s.local_addr().unwrap(),
    peer_addr = %s.peer_addr().unwrap(),
))]
async fn process_socket(mut s: TcpStream) {
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
}
