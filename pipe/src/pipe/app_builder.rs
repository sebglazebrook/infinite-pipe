use super::{App, HistoryLike, InputReaderLike, InputHandlerLike};

pub struct AppBuilder {
    readline: Option<Box<InputReaderLike>>,
    input_handler: Option<Box<InputHandlerLike>>,
    external_history: Option<Box<HistoryLike>>,
}

impl AppBuilder {

    pub fn new() -> Self {
        AppBuilder { 
            readline: None,
            input_handler: None,
            external_history: None,
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

    pub fn build(&mut self) -> App {
        let mut app = App::new();
        app.input_reader = self.readline.take().unwrap();
        app.external_history = self.external_history.take().unwrap();
        app
    }
    
}
