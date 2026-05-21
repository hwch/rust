use igrep::run;
use igrep::Config;
use std::env;
use std::process;
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("解析命令行参数失败: {}", err);
        process::exit(1);
    });
    #[cfg(debug_assertions)]
    dbg!(&config);
    if let Err(err) = run(&config) {
        eprintln!("搜索关键字[{}]失败: {:?}", config.query(), err);
        process::exit(2);
    };
}
