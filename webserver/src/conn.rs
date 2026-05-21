use core::str;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self};
use std::io::{Read, Write};
use std::net::{self, SocketAddr};
use std::sync::Arc;

pub struct Conn {
    c: net::TcpStream,
    addr: SocketAddr,
}
impl Drop for Conn {
    fn drop(&mut self) {
        println!("{}:{}开始关闭链接", self.addr.ip(), self.addr.port());

        self.c
            .shutdown(net::Shutdown::Both)
            .expect("Shutdown botch failed");
        println!("{}:{}关闭了链接", self.addr.ip(), self.addr.port());
    }
}
impl Conn {
    pub fn new(c: net::TcpStream, addr: core::net::SocketAddr) -> Conn {
        Conn { c, addr }
    }

    fn get(&mut self, data: Vec<&str>, route: &HashMap<String, String>) {
        let ver: Vec<_> = data[2].split("/").collect();
        if ver.len() != 2 || ver[1] < "1.1" {
            let e = format!("Error: 版本不能低于1.1");
            eprintln!("{e}");
            let _ = self.c.write(
                format!(
                    "{}{}{}",
                    "HTTP/1.1 500 Internal Server Error\r\n",
                    "Content-Length: 43\r\n\r\n",
                    "<html><body>Version too lower</body></html>"
                )
                .as_bytes(),
            );
            return ();
        }
        let status_line = format!("{} {} {}", data[2], 200, "OK");
        if let Some(filename) = route.get(data[1]) {
            let contents = match fs::read_to_string(filename) {
                Ok(v) => v,
                Err(e) => {
                    let _ = self.c.write(
                        format!(
                            "{}{}{e}{}",
                            "HTTP/1.1 500 Internal Server Error\r\n",
                            "Content-Length: 31\r\n\r\n<html><body>",
                            "</body></html>"
                        )
                        .as_bytes(),
                    );
                    return ();
                }
            };
            let response = format!(
                "{status_line}\r\nContent-Length: {}\r\n\r\n{contents}",
                contents.len()
            );
            println!("{response}");
            self.c.write_all(response.as_bytes()).unwrap();
        } else {
            let _ = self.c.write(
                format!(
                    "{}{}",
                    "HTTP/1.1 404 Not Found\r\nContent-Length: 35\r\n\r\n",
                    "<html><body>Not Found</body></html>"
                )
                .as_bytes(),
            );
            return ();
        }
    }

    fn recv(&mut self) -> Result<String, Box<dyn Error>> {
        let mut buf: String = String::new();
        let mut data: [u8; 8192] = [0; 8192];
        loop {
            match self.c.read(&mut data) {
                Ok(i_size) => {
                    if i_size != 0 {
                        buf = buf
                            + match str::from_utf8(&data[..i_size]) {
                                Ok(v) => v,
                                Err(e) => {
                                    let e = format!("读取数据失败:{e}");
                                    eprintln!("{e}");
                                    return Err(e.into());
                                }
                            };
                    }
                    if i_size < 8192 {
                        break;
                    }
                }
                Err(e) => {
                    let e = format!("读取数据失败:{e}");
                    eprintln!("{e}");
                    return Err(e.into());
                }
            };
        }

        Ok(buf)
    }
    pub fn run(&mut self, route: Arc<HashMap<String, String>>) -> () {
        loop {
            let buf = match self.recv() {
                Ok(v) => {
                    if v.is_empty() {
                        break;
                    } else {
                        v
                    }
                }
                _ => continue,
            };

            let mut lines = buf.lines();
            let first_line = lines.next().unwrap();
            let data: Vec<&str> = first_line
                .splitn(3, ' ')
                .take_while(|l| l.trim().len() != 0)
                .collect();
            if data.len() < 3 {
                let e = format!("Error: 请求格式错误，{:?}", first_line);
                eprintln!("{e}");
                continue;
            }

            match data[0] {
                "GET" => self.get(data, &route),
                _ => {
                    let e = format!("Error: 请求格式错误，{:?}", first_line);
                    eprintln!("{e}");
                    continue;
                }
            }
        }
        ()
    }
}
