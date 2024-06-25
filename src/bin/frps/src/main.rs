use std::{env, process};

use frps::config::Configure;
use frps::server::master::Server;


fn main() {
    let args = env::args().collect();
    let configure = Configure::get(args);

    // println!("{:#?}", configure);

    if let Ok(server) = Server::from(configure) {
        server.start()
    }
    process::exit(1)
}

