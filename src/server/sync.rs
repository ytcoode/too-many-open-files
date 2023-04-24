use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
    process,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

use tracing::{error, info, instrument};

pub fn start(addr: SocketAddr) {
    let listener = TcpListener::bind(addr).expect("TcpListener::bind");

    info!(
        pid = process::id(),
        "Started listening for connections on the address {}.",
        listener.local_addr().expect("listener.local_addr"),
    );

    let counter = Arc::new(AtomicUsize::new(0));

    loop {
        match listener.accept() {
            Ok((s, _)) => {
                let counter = counter.clone();
                thread::spawn(move || handle_client(s, counter));
            }
            Err(e) => {
                error!("An error occurred while calling listener.accept: {}", e);
            }
        }
    }
}

#[instrument(skip_all, fields(
    peer_addr = %s.peer_addr().unwrap(),
    local_addr = %s.local_addr().unwrap(),
))]
fn handle_client(s: TcpStream, counter: Arc<AtomicUsize>) {
    info!(
        "Accepted a connection. The number of currently established connections is {}.",
        counter.fetch_add(1, Ordering::Relaxed) + 1
    );

    for b in s.bytes() {
        match b {
            Ok(_) => break,
            Err(e) => error!(
                "An error occurred while attempting to read a byte from the connection: {}",
                e
            ),
        }
    }

    info!(
        "Closed a connection. The number of currently established connections is {}.",
        counter.fetch_sub(1, Ordering::Relaxed) - 1
    );
}
