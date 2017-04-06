extern crate pipe;

use pipe::AppFactory;


fn main() {
    let _ = AppFactory::create().start();
}

// handle control+c
// output error messages
// update readline session history
// add custom command handling, back, exit
