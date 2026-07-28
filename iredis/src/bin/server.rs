use bytes::Bytes;
use mini_redis::Command::{Get, Set};
use mini_redis::{Command, Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:6379").await?;
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (stream, addr) = server.accept().await?;
        let db = Arc::clone(&db);
        tokio::spawn(async move {
            println!("addr: {addr}");
            process(stream, db).await;
        });
    }
    Ok(())
}

async fn process(stream: TcpStream, db: Db) {
    // 使用 hashmap 来存储 redis 的数据
    // let mut db = HashMap::new();

    // `Connection` 对于 redis 的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    // `Connection` 是在 mini-redis 中定义
    let mut connection = Connection::new(stream);

    loop {
        match connection.read_frame().await {
            Err(e) => {
                eprintln!("read frame error: {:?}", e);
                break;
            }
            Ok(v) => {
                if let Some(frame) = v {
                    println!("GOT: {:?}", frame);
                    do_frame(&mut connection, frame, &db).await;
                } else {
                    println!("Other closed!");
                    break;
                }
            }
        }
    }
}

async fn do_frame(connection: &mut Connection, frame: Frame, db: &Db) {
    let cmd: Command = match Command::from_frame(frame) {
        Ok(v) => v,
        Err(e) => {
            // 出错不关闭连接，只打印错误
            eprintln!("get command error: {:?}", e);
            return;
        }
    };

    let response = match cmd {
        Set(cmd) => {
            // 值被存储为 `Vec<u8>` 的形式
            if let Ok(mut db) = db.lock() {
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            } else {
                eprintln!("get lock on command `SET` failed: {:?}", cmd);
                return;
            }
        }
        Get(cmd) => {
            if let Ok(db) = db.lock() {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` 期待数据的类型是 `Bytes`， 该类型会在后面章节讲解，
                    // 此时，你只要知道 `&Vec<u8>` 可以使用 `into()` 方法转换成 `Bytes` 类型
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            } else {
                eprintln!("get lock on command `GET` failed: {:?}", cmd);
                return;
            }
        }
        cmd => {
            eprintln!("unimplemented {:?}", cmd);
            return;
        }
    };

    // 将请求响应返回给客户端
    if let Err(e) = connection.write_frame(&response).await {
        eprintln!("Send Command failed: {:?}", e);
    };
}
