use pipe::{App, InputReader, ExternalHistory, AppBuilder, CommandRunnerFactory};

pub struct AppFactory;

impl AppFactory {

    pub fn create() -> App {
        let readline = InputReader::new();
        let external_history = ExternalHistory::new("/root/.bash_history");
        let command_runner = CommandRunnerFactory::create();
        AppBuilder::new()
                  .with_readline(readline)
                  .with_external_history(external_history)
                  .with_command_runner(command_runner)
                  .build()
    }
}

