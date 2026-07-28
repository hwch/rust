use bytes::{Buf, BytesMut};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);

    // 创建异步任务，在后台写入数据
    tokio::spawn(async move {
        wr.write_all(b"123456\r\n").await?;
        wr.write_all(b"1234567890\r\n").await?;
        wr.write_all(b"1234567890\r\n").await?;
        wr.write_all(b"1234567890\r\n").await?;
        wr.write_all(b"1234567890\r\n").await?;
        wr.write_all(b"1234567890\r\n").await?;
        wr.write_all(b"1234567890\r\n").await?;
        wr.write_all(b"1234567890\r\n").await?;
        // 有时，我们需要给予 Rust 一些类型暗示，它才能正确的推导出类型
        let _ = wr.shutdown().await;
        Ok::<_, io::Error>(())
    });

    let mut buf = BytesMut::with_capacity(8);

    loop {
        let n = rd.read_buf(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}|{}|{}", buf, buf.len(), buf.capacity());
        buf.advance(buf.len());
    }

    Ok(())
}
