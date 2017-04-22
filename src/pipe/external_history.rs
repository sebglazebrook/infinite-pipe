use rl_sys::history::{listmgmt, histfile};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct ExternalHistory<'a> {
    filepath: &'a str,
}

impl<'a> ExternalHistory<'a> {

    pub fn new(filepath: &'a str) -> Self {
        listmgmt::clear();
        ExternalHistory { filepath: filepath }
    }
}

impl<'a> HistoryLike for ExternalHistory<'a> {

    fn push(&mut self, command: String) {
        let _ = listmgmt::add(&command);
        histfile::append(Some(&Path::new(&self.filepath)), 1).unwrap();
    }

    fn last(&self) -> Option<String> {
        let mut file = File::open(&self.filepath).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        match contents.lines().last() {
            Some(line) => { Some(line.to_string()) },
            None => { None },

        }
    }
}

pub trait HistoryLike {
    fn push(&mut self, command: String);
    fn last(&self) -> Option<String>;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    fn fetch_file_contents<'a>(filepath: &'a str) -> String {
        let mut file = File::open(filepath).unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        contents
    }

    describe! external_history {

        before_each  {
            let history_filepath = "/tmp/test-history";
        }

        describe! push {

            describe! when_the_history_file_is_empty {

                before_each {
                    let _ = File::create(history_filepath).unwrap();
                    let mut external_history = ExternalHistory::new(history_filepath);
                    external_history.push(String::from("ps -ef"));
                }

                it "adds the entry to the history" {
                    let contents = fetch_file_contents(history_filepath);
                    assert_eq!(contents, "ps -ef\n");
                }
            }

            describe! when_the_history_file_has_items {

                before_each {
                    let mut file = File::create(history_filepath).unwrap();
                    let _ = file.write_all(b"ls -la\n");
                    let mut external_history = ExternalHistory::new(history_filepath);
                    external_history.push(String::from("ps -ef"));
                }

                it "appends the entry to the history" {
                    let contents = fetch_file_contents(history_filepath);
                    assert_eq!(contents, "ls -la\nps -ef\n");
                }
            }
        }

        describe! last {

            before_each {
                let history_filepath = "/tmp/test-history";
                let _ = File::create(history_filepath).unwrap();
                let mut external_history = ExternalHistory::new(history_filepath);
            }

            describe! when_the_history_is_empty {

                it "returns None" {
                    assert_eq!(external_history.last(), None);
                }
            }

            describe! when_the_history_has_items {

                before_each {
                    external_history.push(String::from("ps -ef"));
                }

                it "returns the last item" {
                    assert_eq!(external_history.last(), Some(String::from("ps -ef")));
                }
            }
        }
    }
}
