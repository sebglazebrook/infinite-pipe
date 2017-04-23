extern crate pipe;

use pipe::AppFactory;


fn main() {
    let _ = AppFactory::create().start();
}

// output error messages
// update readline session history
