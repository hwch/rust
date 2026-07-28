use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(0, 0, 0, 0),
        6142,
    )))
    .await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            if let Err(e) = io::copy(&mut rd, &mut wr).await {
                eprintln!("failed to copy: {:?}", e);
            }
        });
    }
}
