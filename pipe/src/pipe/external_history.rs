pub struct ExternalHistory;

impl ExternalHistory {

    pub fn new() -> Self {
       ExternalHistory 
    }

}

impl HistoryLike for ExternalHistory {

    fn push(&mut self, command: String) {
    }

    fn last(&self) -> Option<String> {
        None
    }

}

pub trait HistoryLike {
    fn push(&mut self, command: String);
    fn last(&self) -> Option<String>;
}
