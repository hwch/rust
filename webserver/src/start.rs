use std::error::Error;
use webserver::server::Server;
fn main() -> Result<(), Box<dyn Error>> {
    let mut srv = Server::new("config.dat".to_string());
    let fd = srv.init()?;
    // srv.run(fd);
    srv.run_with_pool(fd, 5);
}
