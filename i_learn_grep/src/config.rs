//! 配置信息，包括查询关键字、查询文档路径名、是否忽略大小写

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    /// 根据传入参数生成运行配置参数
    /// # Errors
    /// 1. 在命令行不输入任何参数是方法会失败返回
    /// 2. 再命令行输入不足2个参数时方法也会失败返回
    pub fn build<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        let _ = args.next(); // 忽略程序名
        let mut ignore_case: bool = false;
        let mut query = String::new();
        let mut file_path = String::new();
        for item in args {
            match item.as_str() {
                "-i" => ignore_case = true,
                _ => {
                    if query.is_empty() {
                        query = item;
                    } else {
                        file_path = item;
                    }
                }
            }
        }
        if query.is_empty() {
            return Err("Didn't get a query string");
        };
        if file_path.is_empty() {
            return Err("Didn't get a file path");
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
    /// 返回查询字符串
    pub fn query(&self) -> &str {
        &self.query
    }
    /// 返回查询文件名
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
    /// 返回是否忽略大小写
    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}
