//! 配置信息，包括查询关键字、查询文档路径名、是否忽略大小写

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    /// 根据传入参数生成
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
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
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}
