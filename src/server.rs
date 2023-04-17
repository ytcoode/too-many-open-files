use std::net::SocketAddr;

use tokio::{
    io,
    net::{TcpListener, TcpStream},
};

pub async fn start(addr: SocketAddr) {
    let listener = TcpListener::bind(addr).await.expect("TcpListener::bind");

    println!(
        "Started listening for TCP connections on the address {}",
        listener.local_addr().expect("listener.local_addr")
    );

    loop {
        match listener.accept().await {
            Err(e) => println!("An error occurred while calling listener.accept: {}", e),
            Ok((s, _)) => {
                tokio::spawn(process_socket(s));
            }
        }
    }
}

async fn process_socket(mut s: TcpStream) {
    let addr = format!("{} -> {}", s.peer_addr().unwrap(), s.local_addr().unwrap());
    let (mut r, mut w) = s.split();

    match io::copy(&mut r, &mut w).await {
        Ok(n) => println!("{}: Copied {} bytes", addr, n),
        Err(e) => println!("{}: An error occurred: {}", addr, e),
    }
}
