use std::net::SocketAddr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn start(addr: SocketAddr) {
    loop {
        match TcpStream::connect(addr).await {
            Err(e) => {
                println!("Failed to connect to {}: {}", addr, e);
                break;
            }
            Ok(s) => {
                tokio::spawn(process_socket(s));
            }
        }
    }

    println!("client stopped");
}

async fn process_socket(mut s: TcpStream) {
    let b1 = b"hello";
    let mut b2 = [0; 5];

    s.write_all(b1).await.unwrap();
    s.read_exact(&mut b2).await.unwrap();

    assert_eq!(b1, &b2);

    s.read_u8().await.unwrap();
}
