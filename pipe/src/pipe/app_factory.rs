use pipe::{App, InputReader, InputHandler, ExternalHistory, AppBuilder, StdoutLogger};

pub struct AppFactory;

impl AppFactory {

    pub fn create() -> App {
        let readline = InputReader::new();
        let input_handler = InputHandler::new();
        let external_history = ExternalHistory::new("/root/.bash_history");
        let stdout_logger = StdoutLogger {};
        AppBuilder::new()
                  .with_readline(readline)
                  .with_input_handler(input_handler)
                  .with_external_history(external_history)
                  .with_logger(stdout_logger)
                  .build()
    }
}

