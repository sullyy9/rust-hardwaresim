use std::collections::HashMap;

#[derive(Clone)]
pub enum Command {
    Add(String, String),
    Remove(String, String),
    Quit,

    None,
}

// command:        Last 5 commands input by the user. Element 0 is the most recent.
// valid_commands: Hashmap of valid commands. The key is what the user is expected to enter.
//                 The value consists of a Command enum type and the number of expected arguments.
pub struct Input {
    commands: [Command; 5],
    valid_commands: HashMap<String, (Command, u16)>,
}
impl Input {
    pub fn new() -> Input {
        let mut valid_commands: HashMap<String, (Command, u16)> = HashMap::new();

        valid_commands.insert(
            "add".to_string(),
            (Command::Add("".to_string(), "".to_string()), 2),
        );

        valid_commands.insert(
            "remove".to_string(),
            (Command::Remove("".to_string(), "".to_string()), 2),
        );

        valid_commands.insert("quit".to_string(), (Command::Quit, 0));

        Input {
            commands: [
                Command::None,
                Command::None,
                Command::None,
                Command::None,
                Command::None,
            ],
            valid_commands,
        }
    }

    // Parse a string into a command and its arguments
    pub fn parse(&mut self, input: &String) -> Result<(), ()>{
        // Split the user input, seperating command and arguments
        let split_input: Vec<&str> = input.split_whitespace().collect();
        let input_command = split_input[0];
        let input_args: Vec<&str> = split_input[1..].to_vec();

        // See if there's a valid command
        match self.valid_commands.get(input_command) {
            Some((command, command_args)) => {
                // Check the number of arguments provided
                if input_args.len() < (*command_args).into() {
                    println!("Too few arguments to command: {}", input_command);
                    Err(())
                } else if input_args.len() > (*command_args).into() {
                    println!("Too many arguments to command: {}", input_command);
                    Err(())
                } else {
                    self.commands.rotate_right(1);
                    self.commands[0] = command.to_owned();
                    Input::command_add_args(&mut self.commands[0], input_args);
                    Ok(())
                }
            }
            None => {
                println!("Invalid command: {}", input_command);
                Err(())
            }
        }

    }

    fn command_add_args(command: &mut Command, args: Vec<&str>) {
        match command {
            Command::Add(ref mut arg1, ref mut arg2) => {
                *arg1 = args[0].to_owned();
                *arg2 = args[1].to_owned();
            }
            Command::Remove(ref mut arg1, ref mut arg2) => {
                *arg1 = args[0].to_owned();
                *arg2 = args[1].to_owned();
            }
            Command::Quit => {}
            Command::None => {}
        }
    }
}
