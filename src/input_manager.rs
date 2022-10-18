// Libraries
use std::{thread, time, io::{stdin, stdout, Write}, fmt::Display, str::FromStr, any::TypeId};

const DEFAULT_DELAY: u64 = 35;

/*
#[cfg(target_os = "windows")]
fn cls() {
    Command::new("cls")
        .spawn()
        .expect("Error clearing the screen.");
}
#[cfg(any(target_os = "macos", target_os = "linux"))]
fn cls() {
    Command::new("clear")
        .spawn()
        .expect("Error clearing the screen.");
}
*/

fn flush() {
    stdout().flush().unwrap();
}

pub struct InputManager {
    pub print_delay: time::Duration
}

// Constructor
impl InputManager {
    pub fn new() -> Self {
        Self {
            print_delay: time::Duration::from_millis(DEFAULT_DELAY)
        }
    }
}

// Output to player
impl InputManager {
    // For printing one line, but with no newline at the end
    pub fn print <F: IntoIterator> (&self, text: F)
    where <F as IntoIterator>::Item: Display
    {
        // Print each character one at a time
        text.into_iter().for_each(|char| {
            print!("{}", char);
            // Force stdout to refresh
            flush();
            // Delay between each character
            thread::sleep(self.print_delay);
        });
    }
    // For printing one line with a newline at the end
    pub fn println <F: IntoIterator> (&self, text: F)
        where <F as IntoIterator>::Item: Display
    {
        self.print(text);
        println!("");
    }
    // For printing multiple lines - delay between each line
    pub fn printlns <F: Display> (&self, text: Vec<F>) {
        text.into_iter().for_each(|line| {
            // Print the line
            println!("{}", line);
            // Force stdout to refresh
            flush();
            // Delay between each line
            thread::sleep(self.print_delay * 4);
        })
    }
}

// Input from player
impl InputManager {
    pub fn prompt <T: FromStr + 'static> (&self, prompt: &str, error_msg: &str) -> T {
        // Prompt the player for input, as long as the prompt isn't blank
        if prompt != "" {
            self.println(prompt.chars());
        }

        // Now make the prompt for the player to type into
        let prompt_type: &str;
        // If the prompt type is a string, set prompt_type to text
        if TypeId::of::<T>() == TypeId::of::<String>() {
            prompt_type = "Text";
        // If it's an i32, set prompt_type to number
        } else if TypeId::of::<T>() == TypeId::of::<i32>() {
            prompt_type = "Number";
        } else {
            // Failsafe
            prompt_type = "Unknown";
        }
        // The actual prompt
        let prompt = format!("({}) > ", prompt_type);

        // Now loop until the player types something that can be converted into the type we need
        loop {
            // Make a prompt
            print!("{}", prompt);
            flush();
        
            // For storing the player input
            let mut input = String::new();

            // Get the input they type
            stdin().read_line(&mut input).unwrap();

            // Match it to make sure it's the correct type
            match input.trim().parse::<T>() {
                Ok(result) => break result,
                Err(_) => self.println(error_msg.chars())
            }
        }
    }

    // Print a list of options, then let the user choose one, and return the chosen option
    pub fn options_prompt(&self, prompt: &str, raw_options: Vec<&str>) -> String {
        // A new vec holding the options, so we can modify them
        let mut options = Vec::<String>::new();
        // Add a number before each option (1., 2., etc)
        raw_options
            .iter()
            .fold(1, |index, option| {
                // Push the number of the option & the option text to options
                options.push(format!("{}. {}", index, option));

                // Increase the counter, so the next option's number is 1 higher
                return index + 1;
            });
        
        // Print the prompt
        println!("{}", prompt);
        // Print the choices
        self.printlns(options);

        // Loop getting player input until they choose a valid option, then return it
        loop {
            // Get player input
            let input = self.prompt::<i32>(
                "",
                "Please type the number of the option you want."
            );

            // Now make sure the input is actually one of the options we printed
            match raw_options.get((input - 1) as usize) {
                Some(choice) => break String::from(*choice),
                None => self.println("Please choose an option that was printed.".chars())
            }
        }
    }
}