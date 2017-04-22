use super::{App, HistoryLike, InputReaderLike, CommandRunnerLike};

pub struct AppBuilder {
    readline: Option<Box<InputReaderLike>>,
    external_history: Option<Box<HistoryLike>>,
    command_runner: Option<Box<CommandRunnerLike>>,
}

impl AppBuilder {

    pub fn new() -> Self {
        AppBuilder { 
            readline: None,
            external_history: None,
            command_runner: None,
        }
    }

    pub fn with_readline<T: InputReaderLike + 'static>(&mut self, readline: T) -> &mut AppBuilder {
        self.readline = Some(Box::new(readline));
        self
    }

    pub fn with_external_history<T:HistoryLike + 'static>(&mut self, external_history: T) -> &mut Self {
        self.external_history = Some(Box::new(external_history));
        self
    }

    pub fn with_command_runner<T:CommandRunnerLike+ 'static>(&mut self, command_runner: T) -> &mut Self {
        self.command_runner = Some(Box::new(command_runner));
        self
    }

    pub fn build(&mut self) -> App {
        let external_history = self.external_history.take().unwrap();
        let input_reader = self.readline.take().unwrap();
        let command_runner = self.command_runner.take().unwrap();
        App {
            external_history: external_history,
            input_reader: input_reader,
            command_runner: command_runner,
            line_index: 1,
        }
    }
    
}
