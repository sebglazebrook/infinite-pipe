extern crate pipe;

use pipe::AppFactory;


fn main() {
    let exit_code = AppFactory::create().start();
}

// handle the resultant_command
// handle control+c
// output error messages
// update readline session history
// add custom command handling, back, exit
// update external history
