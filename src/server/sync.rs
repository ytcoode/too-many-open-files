use std::net::{SocketAddr, TcpListener};

use tracing::error;

pub fn start(addr: SocketAddr) {
    let listener = TcpListener::bind(addr).expect("TcpListener::bind");

    info!(
        "Started listening for TCP connections on the address {}",
        listener.local_addr().expect("listener.local_addr")
    );

    loop {
        match listener.accept() {
            Ok((s, _)) => (),
            Err(error) => error!(%error, "An error occurred while calling listener.accept"),
        }
    }
}
