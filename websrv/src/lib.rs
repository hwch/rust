use std::{error::Error, net::TcpListener};

pub struct Runtime {
    ip: String,
    port: i32,
}

impl Runtime {
    pub fn build(args: Vec<String>) -> Result<Runtime, &'static str> {
        let mut ip = String::from("127.0.0.1");
        let mut port = 8080;
        for (index, value) in args.iter().map(|e| e.as_str()).enumerate() {
            match value {
                "-h" => {
                    if index >= args.len() || args[index + 1].starts_with("-") {
                        return Err("请输入主机地址");
                    }
                    ip = String::from(&args[index + 1]);
                }
                "-p" => {
                    if index >= args.len() || args[index + 1].starts_with("-") {
                        return Err("请输入主机端口号");
                    }
                    let Ok(_port) = (&args[index + 1]).parse::<i32>() else {
                        return Err("请输入有效的数字");
                    };

                    if _port < 0 || _port > 65535 {
                        return Err("输入的端口号超过有效表示范围(1~65535)");
                    } else {
                        port = _port;
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        Ok(Runtime { ip, port })
    }
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let listener = match TcpListener::bind(format!("{}:{}", self.ip, self.port)) {
            Ok(v) => v,
            Err(e) => {
                return Err(Box::new(e));
            }
        };
        for income in listener.incoming() {
            let stream = income.unwrap();
            println!("Connection established!!");
        }
        Ok(())
    }
}
