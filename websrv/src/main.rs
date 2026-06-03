use std::env;
use std::net::TcpListener;

fn main() {
    let iargs: Vec<String> = env::args().collect();
    let mut host = String::from("127.0.0.1");
    let mut port = 8080;
    for (index, value) in iargs.iter().map(|e| e.as_str()).enumerate() {
        match value {
            "-h" => {
                if index >= iargs.len() || iargs[index + 1].starts_with("-") {
                    eprintln!("请输入主机地址");
                    return;
                }
                host = String::from(&iargs[index + 1]);
            }
            "-p" => {
                if index >= iargs.len() || iargs[index + 1].starts_with("-") {
                    eprintln!("请输入主机端口号");
                    return;
                }
                let Ok(_port) = (&iargs[index + 1]).parse::<i32>() else {
                    eprintln!("请输入有效的数字");
                    return;
                };

                if _port < 0 || _port > 65535 {
                    eprintln!("输入的端口号超过有效表示范围(1~65535)");
                    return;
                } else {
                    port = _port;
                }
            }
            _ => {
                continue;
            }
        }
    }
    let listener = match TcpListener::bind(format!("{host}:{port}")) {
        Ok(e) => e,
        Err(e) => {
            println!("创建监听失败：{:?}", e);
            return;
        }
    };
    for income in listener.incoming() {
        let stream = income.unwrap();
        println!("Connection established!!");
    }
}
