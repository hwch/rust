use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/// 管理任务可以使用该发送端将命令执行的结果传回给发出命令的任务
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    // 创建到服务器的连接
    let mut client = match client::connect("127.0.0.1:6379").await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("connect server failed: {:?}", e);
            return;
        }
    };

    // 生成两个任务，一个用于获取 key, 一个用于设置 key
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        if let Err(e) = tx
            .send(Command::Get {
                key: "foo".to_string(),
                resp: resp_tx,
            })
            .await
        {
            eprintln!("get `foo` failed: {:?}", e);
            return;
        }
        match resp_rx.await {
            Ok(v) => println!("t1 got = {:?}", v),
            Err(_) => println!("the t1's sender dropped"),
        }
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        if let Err(e) = tx2
            .send(Command::Set {
                key: "foo".to_string(),
                val: "bar".into(),
                resp: resp_tx,
            })
            .await
        {
            eprintln!("set `bar` to `foo` failed: {:?}", e);
            return;
        }
        match resp_rx.await {
            Ok(v) => println!("t2 got = {:?}", v),
            Err(_) => println!("the t2's sender dropped"),
        }
    });

    let manager = tokio::spawn(async move {
        while let Some(v) = rx.recv().await {
            println!("received cmd: {:?}", v);
            match v {
                Command::Get { key, resp } => {
                    if let Err(e) = resp.send(client.get(&key).await) {
                        eprintln!("cmd get response failed: {:?}", e);
                        continue;
                    }
                }
                Command::Set { key, val, resp } => {
                    if let Err(e) = resp.send(client.set(&key, val).await) {
                        eprintln!("cmd set response failed: {:?}", e);
                        continue;
                    }
                }
            }
        }
    });

    t1.await.expect("t1 failed");
    t2.await.expect("t2 failed");
    manager.await.expect("manager failed");
}
