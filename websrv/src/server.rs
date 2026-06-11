use crate::IError;
use crate::ThreadPool;

use std::fs;
use std::io::{BufReader, prelude::*};
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;
use std::thread;
use std::time;

pub type IReturn<I> = std::result::Result<I, IError>;

pub struct Server {
    ip: String,
    port: i32,
    pool: ThreadPool,
}

impl Server {
    // 最大错误数
    const MAX_ERR: i32 = 10;
    // 正常响应
    const STATUS_OK: &str = "HTTP/1.1 200 OK";
    // 未找到响应
    const STATUS_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";
    // 线程池最大线程数
    const MAX_POOL: usize = 1024;
    // 线程池默认线程数
    const DEFAULT_POOL: usize = 10;
    /// 解析命令行参数并返回Server实体且初始化线程池
    pub fn build(args: Vec<String>) -> IReturn<Server> {
        let mut ip = String::from("127.0.0.1");
        let mut port = 8080;
        let mut pool_size = Server::DEFAULT_POOL;
        for (index, value) in args.iter().map(|e| e.as_str()).enumerate() {
            match value {
                "-j" => {
                    if index + 1 >= args.len() || args[index + 1].starts_with("-") {
                        return Err("请输入最大线程数".into());
                    }
                    let _pool_size = if let Ok(v) = args[index + 1].parse::<usize>() {
                        v
                    } else {
                        return Err("请输入有效的数字".into());
                    };
                    if _pool_size > Server::MAX_POOL {
                        pool_size = Server::MAX_POOL;
                    } else {
                        pool_size = _pool_size;
                    }
                }
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
        let pool = ThreadPool::build(pool_size)?; // 初始化线程池
        Ok(Server { ip, port, pool })
    }
    /// 服务器监听运行，在incoming连续出错MAX_ERR次后服务器会退出
    ///
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
                    if err_count == Server::MAX_ERR {
                        return Err(e.into());
                    }
                    eprintln!("获取连接发生错误: {:?}", e);
                    continue;
                }
            };

            // thread::scope(|s| {
            //     s.spawn(|| {
            //         let addr = match stream.peer_addr() {
            //             Ok(v) => v.to_string(),
            //             _ => "Unkown Host".to_string(),
            //         };
            //         let res = self.handle_connection(stream);
            //         if res.is_err() {
            //             eprintln!("连接{}发生错误: {:?}", addr, res);
            //         }
            //     });
            // });

            self.pool.execute(move || {
                let addr = match stream.peer_addr() {
                    Ok(v) => v.to_string(),
                    _ => "Unkown Host".to_string(),
                };
                let res = Server::handle_connection(stream);
                if res.is_err() {
                    eprintln!("连接{}发生错误: {:?}", addr, res);
                }
            });
        }
        Ok(())
    }

    fn handle_connection(mut stream: TcpStream) -> IReturn<()> {
        let mut buf_reader = BufReader::new(&stream);

        let mut buf: String = String::new();
        buf_reader.read_line(&mut buf)?;
        let elems: Vec<_> = buf.trim().split_whitespace().collect();

        if elems.len() < 3 {
            eprintln!("{buf}");
            return Err(IError::InvalidRequestError);
        }

        match elems[0] {
            "GET" => Server::get(elems[1], &mut stream)?,
            "POS" => Server::post(elems[1], &mut stream)?,
            _ => {
                eprintln!("{buf}");
                return Err(IError::InvalidRequestError);
            }
        }

        Ok(())
    }

    fn get(uri: &str, stream: &mut TcpStream) -> IReturn<()> {
        let (filename, status) = match uri {
            "/" => ("hello.html", Server::STATUS_OK),
            "/sleep" => {
                thread::sleep(time::Duration::from_secs(10));
                ("hello.html", Server::STATUS_OK)
            }
            _ => ("404.html", Server::STATUS_NOT_FOUND),
        };

        let contents = fs::read_to_string(filename)?;
        let respone = format!(
            "{status}\r\nContent-Length: {}\r\n\r\n{contents}",
            contents.len()
        );
        stream.write_all(respone.as_bytes())?;
        println!("done");
        Ok(())
    }
    fn post(_: &str, _: &mut TcpStream) -> IReturn<()> {
        Ok(())
    }
}
