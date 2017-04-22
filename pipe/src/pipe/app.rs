use pipe::{InputReaderLike, HistoryLike, CommandRunner, CommandResponse};

pub struct App {
    pub line_index: usize,
    pub input_reader: Box<InputReaderLike>,
    pub external_history: Box<HistoryLike>,
    pub command_runner: Box<CommandRunner>,
}

impl App {

    pub fn start(&mut self) -> usize {
        loop {
            match self.read_input() {
                Err(error_message) => { break; },
                Ok(input) => { 
                    match self.command_runner.next_command(input) {
                        CommandResponse::Exit => { break; }
                        _ => {  }
                    }
                }
            }
        }
        self.external_history.push(self.command_runner.resultant_command());
        0 // TODO return a real error code
    }

    // private //
    
    fn read_input(&mut self) -> Result<String, &'static str> {
        let result = self.input_reader.read_line(self.line_index);
        self.line_index += 1;
        result
    }
}

// it reads a line of input
//
// context when the input reading is successful
//   it sends the input to the command runner for execution
//
//   context when the command runner's result is Exit
//     it updates the external history
//
//   context when the command runner's result is NOT Exit
//     it reads the next line of input
//
// context when the input reading is unsuccessful
