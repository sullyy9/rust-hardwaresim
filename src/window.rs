use crossterm::{cursor, terminal, ExecutableCommand};
use io::Write;
use std::io;

const WINDOW_COLS: u16 = 200;
const WINDOW_ROWS: u16 = 50;

const INPUT_HISTORY_LENGTH: usize = 10;

pub struct Window {
    stdout: io::Stdout,
    stdin: io::Stdin,

    input_history: [String; INPUT_HISTORY_LENGTH],
}
impl Window {
    pub fn new() -> Window {
        let input_history: [String; INPUT_HISTORY_LENGTH] = Default::default();

        Window {
            stdin: io::stdin(),
            stdout: io::stdout(),

            input_history,
        }
    }

    // Read from stdin and return the string.
    pub fn read(&mut self, buf: &mut String) {
        match self.stdin.read_line(buf) {
            Ok(_ok) => {
                if buf != "\u{a}" {
                    self.input_history.rotate_right(1);
                    self.input_history[0] = buf.to_owned();
                } else {
                    *buf = "a".to_string();
                }
            }
            Err(_error) => {
                println!("ERR: window - read");
            }
        }
    }

    pub fn draw(&mut self) {
        // Clear the screen and set the terminal size
        self.clear();
        self.set_size(WINDOW_COLS, WINDOW_ROWS);

        // Draw component view
        self.move_cursor(0, 30);
        println!("--------------------------------------------------");

        // Draw command view with input history
        for i in self. input_history.iter().rev() {
            if i != "" {
                println!("{}", i);
            }
        }

        print!("command: ");
        match self.stdout.flush() {
            Ok(_ok) => {}
            Err(_error) => {
                println!("ERR: window - flush stdout");
            }
        }
    }

    fn set_size(&mut self, columns: u16, rows: u16) {
        match self.stdout.execute(terminal::SetSize(columns, rows)) {
            Ok(_ok) => {}
            Err(_error) => {
                println!("ERR: window - set_size");
            }
        }
    }

    fn move_cursor(&mut self, column: u16, row: u16) {
        match self.stdout.execute(cursor::MoveTo(column, row)) {
            Ok(_ok) => {}
            Err(_error) => {
                println!("ERR: window - move_cursor");
            }
        }
    }

    fn clear(&mut self) {
        match self.stdout.execute(cursor::MoveTo(0, 0)) {
            Ok(_ok) => {}
            Err(_error) => {
                println!("ERR: window - clear");
            }
        }

        match self
            .stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
        {
            Ok(_ok) => {}
            Err(_error) => {
                println!("ERR: window - clear");
            }
        }
    }
}
