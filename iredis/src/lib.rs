//!
#[cfg(all(feature = "vec", feature = "bytes"))]
compile_error!("feature `vec` and feature `bytes` can not be enable at same time");

#[cfg(not(any(feature = "vec", feature = "bytes")))]
compile_error!("feature `vec` and feature `bytes` must be enable someone");

use tokio::net::TcpStream;
use tokio::time::timeout;

pub mod cmd;
pub mod conn;
pub mod frame;
pub mod types;

pub use cmd::Command;
pub use conn::Connection;
pub use frame::Frame;
pub use types::Db;
pub use types::Error;

pub async fn process(stream: TcpStream, db: Db) {
    // 使用 hashmap 来存储 redis 的数据
    // let mut db = HashMap::new();

    // `Connection` 对于 redis 的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    // `Connection` 是在 mini-redis 中定义
    let mut connection = Connection::new(stream);
    loop {
        match timeout(std::time::Duration::from_secs(2), connection.read_frame()).await {
            Err(_) => {
                //timeout
                let _ = connection.flush(true).await;
                break;
            }
            Ok(v) => match v {
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
            },
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
        Command::Set(cmd) => {
            // 值被存储为 `Vec<u8>` 的形式
            if let Ok(mut db) = db.lock() {
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            } else {
                eprintln!("get lock on command `SET` failed: {:?}", cmd);
                return;
            }
        }
        Command::Get(cmd) => {
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
        _ => {
            eprintln!("unimplemented {:?}", cmd);
            return;
        }
    };

    // 将请求响应返回给客户端
    if let Err(e) = connection.write_frame(&response).await {
        eprintln!("Send Command failed: {:?}", e);
    };
}
