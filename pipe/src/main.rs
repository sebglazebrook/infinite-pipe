extern crate pipe;

use pipe::App;


fn main() {
    let exit_code = App::new().start();
}

// handle the resultant_command
// handle control+c
// output error messages
// update readline session history
// add custom command handling, back, exit
// update external history
