use crate::IError;
use std::fs;
use std::io::{BufReader, prelude::*};
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;

pub type IReturn<I> = std::result::Result<I, IError>;

pub struct Runtime {
    ip: String,
    port: i32,
}

impl Runtime {
    const MAX_ERR: i32 = 10;
    const SATUS_OK: &str = "HTTP/1.1 200 OK";
    pub fn build(args: Vec<String>) -> IReturn<Runtime> {
        let mut ip = String::from("127.0.0.1");
        let mut port = 8080;
        for (index, value) in args.iter().map(|e| e.as_str()).enumerate() {
            match value {
                "-h" => {
                    if index + 1 >= args.len() || args[index + 1].starts_with("-") {
                        return Err("请输入主机地址".into());
                    }
                    ip = String::from(&args[index + 1]);
                }
                "-p" => {
                    if index + 1 >= args.len() || args[index + 1].starts_with("-") {
                        return Err("请输入主机端口号".into());
                    }
                    let Ok(_port) = (&args[index + 1]).parse::<i32>() else {
                        return Err("请输入有效的数字".into());
                    };

                    if _port < 0 || _port > 65535 {
                        return Err("输入的端口号超过有效表示范围(1~65535)".into());
                    } else {
                        port = _port;
                    }
                }
                "--help" => {
                    println!("{}\n\t-p 监听端口号\n\t-h 主机地址", args.get(0).unwrap());
                    process::exit(0);
                }
                _ => {
                    continue;
                }
            }
        }
        Ok(Runtime { ip, port })
    }
    pub fn run(&self) -> IReturn<()> {
        let mut err_count = 0;
        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port))?;
        for income in listener.incoming() {
            let stream = match income {
                Ok(v) => {
                    err_count = 0;
                    v
                }
                Err(e) => {
                    err_count += 1;
                    if err_count == Runtime::MAX_ERR {
                        return Err(e.into());
                    }
                    eprintln!("获取连接发生错误: {:?}", e);
                    continue;
                }
            };
            let addr = match stream.peer_addr() {
                Ok(v) => v.to_string(),
                _ => "Unkown Host".to_string(),
            };
            let res = self.handle_connection(stream);
            if res.is_err() {
                eprintln!("连接{}发生错误: {:?}", addr, res);
            }
        }
        Ok(())
    }

    fn handle_connection(&self, mut stream: TcpStream) -> IReturn<()> {
        let buf_reader = BufReader::new(&stream);

        let mut http_request = vec![];
        for line in buf_reader.lines() {
            let l = line?;
            if l.is_empty() {
                break;
            }
            http_request.push(l);
        }

        // println!("Request: {http_request:#?}");

        let contents = fs::read_to_string("hello.html")?;

        let respone = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{contents}",
            Runtime::SATUS_OK,
            contents.len()
        );
        stream.write_all(respone.as_bytes())?;
        Ok(())
    }
}
