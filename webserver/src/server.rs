use crate::conn::Conn;
use crate::thread_pool::ThreadPool;
use std::collections::HashMap;
use std::net::TcpListener;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::thread;
use std::{error::Error, fs};

struct RouteType {
    c: HashMap<String, String>,
}
impl RouteType {
    pub fn new() -> RouteType {
        RouteType { c: HashMap::new() }
    }
}
impl Deref for RouteType {
    fn deref(&self) -> &Self::Target {
        &self.c
    }

    type Target = HashMap<String, String>;
}

impl DerefMut for RouteType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.c
    }
}

pub struct Server {
    filename: String,
    port: i16,
    addr: String,
    route: Arc<HashMap<String, String>>,
}

impl Server {
    pub fn new(filename: String) -> Server {
        Server {
            filename,
            port: 0,
            addr: "".to_string(),
            route: Arc::new(HashMap::new()),
        }
    }

    fn get_info(&mut self) -> Result<(), Box<dyn Error>> {
        let data = fs::read_to_string(self.filename.clone())?;
        for line in data.split("\n") {
            let line = line.trim_start();
            if line.len() == 0 || line.starts_with('#') {
                continue;
            }
            let values: Vec<&str> = line
                .splitn(2, '=')
                .map(|l| l.trim_start())
                .take_while(|l| l.len() != 0)
                .collect();
            if values.len() != 2 {
                return Err(format!("Key[{:?}]没有对应的值", values.get(0)).into());
            }
            match values[0].trim_end() {
                "server.port" => {
                    self.port = match values[1].parse() {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(format!("解析Key[{:?}]对应的值失败: {e}", values[0]).into())
                        }
                    }
                }
                "server.ip" => self.addr = values[1].to_string(),
                "route.path" => {
                    let route_path = values[1].to_string();
                    self.route_init(route_path)?;
                }
                _ => return Err(format!("Key[{:?}]未定义", values.get(0)).into()),
            }
        }
        Ok(())
    }

    fn route_init(&mut self, route_path: String) -> Result<(), Box<dyn Error>> {
        let mut route: HashMap<String, String> = HashMap::new();
        let data = fs::read_to_string(route_path).expect("读取路由配置文件错误");

        for line in data.split('\n') {
            let line = line.trim();
            if line.len() == 0 || line.starts_with('#') {
                continue;
            }
            let data: Vec<_> = line.split(' ').filter(|s| !s.is_empty()).collect();
            if data.len() != 2 {
                return Err(format!("参数格式错误").into());
            }

            route.insert(data[0].to_string(), data[1].to_string());
        }
        self.route = Arc::new(route);
        Ok(())
    }
    pub fn init(&mut self) -> Result<TcpListener, Box<dyn Error>> {
        self.get_info()?;
        match TcpListener::bind(format!("{}:{}", self.addr, self.port)) {
            Ok(fd) => return Ok(fd),
            Err(e) => {
                return Err(format!("在端口{}创建监听失败: {e}", self.port).into());
            }
        };
    }
    pub fn run_with_pool(&self, listener: TcpListener, pool_count: usize) -> ! {
        let pool = ThreadPool::new(pool_count);
        loop {
            let (client, addr) = match listener.accept() {
                Ok((client, addr)) => {
                    println!("来自{:?}:{}连接成功", addr.ip(), addr.port());
                    (client, addr)
                }
                Err(e) => {
                    eprintln!("接受连接失败: {e}");
                    continue;
                }
            };

            let route = self.route.clone();

            pool.execute(move || {
                let mut client = Conn::new(client, addr);
                client.run(route);
            });
        }
    }
    pub fn run(&self, listener: TcpListener) -> ! {
        loop {
            let (client, addr) = match listener.accept() {
                Ok((client, addr)) => {
                    println!("来自{:?}:{}连接成功", addr.ip(), addr.port());
                    (client, addr)
                }
                Err(e) => {
                    eprintln!("接受连接失败: {e}");
                    continue;
                }
            };

            let route = self.route.clone();

            thread::spawn(move || {
                let mut client = Conn::new(client, addr);
                client.run(route);
            });
        }
    }
}
