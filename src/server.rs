use std::net::SocketAddr;

mod r#async;
mod sync;

pub fn start(addr: SocketAddr, r#async: bool) {
    if r#async {
        tokio::task::spawn(r#async::start(addr));
    } else {
        tokio::task::spawn_blocking(move || sync::start(addr));
    }
}
