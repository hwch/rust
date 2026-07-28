use bytes::{Buf, BytesMut};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = BytesMut::with_capacity(4096);
            let window = [0x0d, 0x0a];
            let (mut rd, mut wr) = socket.split();
            loop {
                match rd.read_buf(&mut buf).await {
                    Ok(n) => {
                        if n == 0 {
                            eprintln!("Other closed");
                            return;
                        }
                        while let Some(pos) = buf.windows(window.len()).position(|x| x == window) {
                            eprintln!(
                                "Write: {:?}",
                                str::from_utf8(&buf[..pos + 2]).expect("无效的UTF8字符串")
                            );
                            if let Err(e) = wr.write(&mut buf[..pos + 2]).await {
                                eprintln!("Write error: {:?}", e);
                                return;
                            }

                            buf.advance(pos + 2); // 加上回车换行
                        }
                    }
                    Err(e) => {
                        eprintln!("Read error: {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
