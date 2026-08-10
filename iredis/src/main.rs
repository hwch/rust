use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:6379").await?;
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (stream, addr) = server.accept().await?;
        let clone_db = Arc::clone(&db);
        tokio::spawn(async move {
            println!("addr: {addr}");
            iredis::process(stream, clone_db).await;
        });
    }
}
