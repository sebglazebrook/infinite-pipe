use rl_sys::readline;

#[derive(Clone)]
pub struct InputReader;

impl InputReader {

    pub fn new() -> Self {
        InputReader {}
    }

}

impl InputReaderLike for InputReader {

    fn read_line(&self, string: String) -> Result<String, String> {
        match readline::readline("") {
            Ok(Some(command)) => { Ok(command.to_string()) }
            Ok(None) => { Err("die".to_string()) },  // user entered ctrl-d
            Err(e) => { Err("an error occurred".to_string()) }
        }
    }
}

#[derive(Mock)]
pub trait InputReaderLike {
    fn read_line(&self, string: String) -> Result<String, String>;
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
