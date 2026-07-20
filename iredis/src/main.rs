use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:6379").await?;
    loop {
        let (stream, addr) = server.accept().await?;
        tokio::spawn(async move {
            println!("addr: {addr}");
            process(stream).await;
        });
    }
    Ok(())
}

async fn process(stream: TcpStream) {
    // `Connection` 对于 redis 的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    // `Connection` 是在 mini-redis 中定义
    let mut connection = Connection::new(stream);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // 回复一个错误
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
