use std::net::SocketAddr;

use tokio::{
    io,
    net::{TcpListener, TcpStream},
};
use tracing::{debug, error, info, instrument};

pub async fn start(addr: SocketAddr) {
    let listener = TcpListener::bind(addr).await.expect("TcpListener::bind");

    info!(
        "Started listening for TCP connections on the address {}",
        listener.local_addr().expect("listener.local_addr")
    );

    let mut err_count = 0;
    let mut ok_count = 0;

    loop {
        match listener.accept().await {
            Err(e) => {
                println!("An error occurred while calling listener.accept: {}", e);
                err_count += 1;
            }
            Ok((s, _)) => {
                tokio::spawn(process_socket(s));
                ok_count += 1;
            }
        }
        println!("ok: {}, err: {}", ok_count, err_count);
    }
}

#[instrument(skip_all, fields(
    peer_addr = %s.peer_addr().unwrap(),
    local_addr = %s.local_addr().unwrap(),
))]
async fn process_socket(mut s: TcpStream) {
    debug!("new connection");
    let (mut r, mut w) = s.split();

    match io::copy(&mut r, &mut w).await {
        Ok(n) => debug!("Copied {} bytes", n),
        Err(e) => error!("An error occurred: {}", e),
    }
}
