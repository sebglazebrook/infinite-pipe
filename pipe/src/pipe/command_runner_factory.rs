use pipe::{CommandRunner, InputHandler, StdoutLogger};

pub struct CommandRunnerFactory;

impl CommandRunnerFactory {

    pub fn create() -> CommandRunner {
        let input_handler = Box::new(InputHandler::new());
        let stdout_logger = Box::new(StdoutLogger {});
        CommandRunner {
            inputs:  vec![],
            outputs: vec![],
            logger: stdout_logger,
            input_handler: input_handler,
        }
    }
}
