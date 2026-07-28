use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0_u8; 1024];
            let mut s = Vec::with_capacity(64);
            let (mut rd, mut wr) = socket.split();
            loop {
                match rd.read(&mut buf[..]).await {
                    Ok(n) => {
                        if n == 0 {
                            eprintln!("Other closed");
                            return;
                        }
                        s.append(&mut (&buf[..n]).to_vec());
                        if s.ends_with(&[0x0d, 0x0a]) {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Read error: {:?}", e);
                        return;
                    }
                }
            }
            if let Err(e) = wr.write_all(&s).await {
                eprintln!("Write error: {:?}", e);
                return;
            }
        });
    }
}
