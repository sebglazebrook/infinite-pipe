use rl_sys::history::{listmgmt, histfile};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct ExternalHistory {
    filepath: String,
}

impl ExternalHistory {

    pub fn new(filepath: &'static str) -> Self {
        listmgmt::clear();
        ExternalHistory { filepath: filepath.to_string() }
    }
}

impl HistoryLike for ExternalHistory {

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

    #[test]
    fn when_a_command_is_pushed_on_it_gets_added_to_the_history() {
        let history_filepath = "/tmp/test-history";
        let _ = File::create(history_filepath).unwrap();
        let mut external_history = ExternalHistory::new(history_filepath.clone());
        external_history.push(String::from("ps -ef"));
        let mut file = File::open(history_filepath).unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        assert_eq!(contents, "ps -ef\n");
    }

    #[test]
    fn when_the_history_already_has_items_new_items_get_appended() {
        let history_filepath = "/tmp/test-history-two";
        {
            let mut file = File::create(history_filepath).unwrap();
            let _ = file.write_all(b"ls -la\n");
        }
        let mut external_history = ExternalHistory::new(history_filepath.clone());
        external_history.push(String::from("ps -ef"));

        let mut file = File::open(history_filepath).unwrap();

        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        assert_eq!(contents, "ls -la\nps -ef\n");
    }

    #[test]
    fn when_the_history_has_items_last_returns_the_last_item() {
        let history_filepath = "/tmp/test-history-three";
        File::create(history_filepath).unwrap();

        let mut external_history = ExternalHistory::new(history_filepath.clone());
        external_history.push(String::from("ps -ef"));

        assert_eq!(external_history.last(), Some(String::from("ps -ef")));
    }

    #[test]
    fn when_the_history_has_no_items_last_returns_nothing() {
        let history_filepath = "/tmp/test-history-four";
        File::create(history_filepath).unwrap();

        let external_history = ExternalHistory::new(history_filepath.clone());

        assert_eq!(external_history.last(), None);
    }
}
