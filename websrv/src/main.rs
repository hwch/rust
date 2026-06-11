use std::env;
use std::process;
use websrv;

fn main() {
    let iargs: Vec<String> = env::args().collect();
    let runtime = websrv::Server::build(iargs).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(-1);
    });
    runtime.run().unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(-2);
    });
}
