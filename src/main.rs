extern crate pipe;

use pipe::AppFactory;
use std::process;


fn main() {
    let exit_code = AppFactory::create().start();
    process::exit(exit_code);
}

// update readline session history
