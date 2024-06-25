mod libs;

use std::env;
use libs::config::Configure;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configure = Configure::get(args);


}

