use pipe::{App, InputReader, InputHandler, ExternalHistory, AppBuilder};

pub struct AppFactory;

impl AppFactory {

    pub fn create() -> App {
        let readline = InputReader::new();
        let input_handler = InputHandler::new(String::new());
        let external_history = ExternalHistory::new("TODO");
        AppBuilder::new()
                  .with_readline(readline)
                  .with_input_handler(input_handler)
                  .with_external_history(external_history)
                  .build()
    }
}

