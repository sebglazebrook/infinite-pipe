use super::{App, HistoryLike, InputReaderLike, InputHandlerLike, LoggerLike};

pub struct AppBuilder {
    readline: Option<Box<InputReaderLike>>,
    input_handler: Option<Box<InputHandlerLike>>,
    external_history: Option<Box<HistoryLike>>,
    logger: Option<Box<LoggerLike>>,
}

impl AppBuilder {

    pub fn new() -> Self {
        AppBuilder { 
            readline: None,
            input_handler: None,
            external_history: None,
            logger: None,
        }
    }

    pub fn with_readline<T: InputReaderLike + 'static>(&mut self, readline: T) -> &mut AppBuilder {
        self.readline = Some(Box::new(readline));
        self
    }

    pub fn with_input_handler<T:InputHandlerLike  + 'static>(&mut self, input_handler: T) -> &mut Self {
        self.input_handler = Some(Box::new(input_handler));
        self
    }

    pub fn with_external_history<T:HistoryLike + 'static>(&mut self, external_history: T) -> &mut Self {
        self.external_history = Some(Box::new(external_history));
        self
    }

    pub fn with_logger<T:LoggerLike + 'static>(&mut self, logger: T) -> &mut Self {
        self.logger = Some(Box::new(logger));
        self
    }

    pub fn build(&mut self) -> App {
        let external_history = self.external_history.take().unwrap();
        let input_reader = self.readline.take().unwrap();
        let input_handler = self.input_handler.take().unwrap();
        let logger = self.logger.take().unwrap();
        App {
            inputs: vec![],
            outputs: vec![],
            external_history: external_history,
            input_reader: input_reader,
            input_handler: input_handler,
            logger: logger,
            line_index: 1,
        }
    }
    
}
