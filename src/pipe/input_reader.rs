use rl_sys::readline;

#[derive(Clone)]
pub struct InputReader;

impl InputReader {

    pub fn new() -> Self {
        InputReader {}
    }

}

impl InputReaderLike for InputReader {

    fn read_line(&self, index: usize) -> Result<String, &'static str> {
        match readline::readline("c:\\> ") {
            Ok(Some(command)) => { Ok(command.to_string()) }
            Ok(None) => { Err("die") },  // user entered ctrl-d
            Err(_) => { Err("an error occurred") }
        }
    }
}

#[derive(Mock)]
pub trait InputReaderLike {
    fn read_line(&self, index: usize) -> Result<String, &'static str>;
}

#[cfg(test)]
mod test {
    use super::*;

    describe! input_reader {

        describe! read_line {

            describe! when_the_user_enters_in_a_command {
                it "returns it" {
                }
            }

            describe! when_the_user_enters_an_emtpy_string {

                it "returns an error" {
                }
            }

            describe! when_an_internal_error_occurrs {

                it "returns an error" {
                }
            }
        }
    }
}
//
    //#[test]
    //fn it_updates_the_internal_history_after_each_successful_command() {
    //}

    //#[test]
    //fn it_updates_the_internal_history_after_each_non_successful_command() {
    //}
