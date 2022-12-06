use super::IOManager;
use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

// Input from player
impl IOManager {
    /// Prompt player for input, and return a trimmed version of that input.
    pub fn prompt_raw(&self, prompt_text: &str) -> String {
        // Check if the cursor is hidden - if it is, we should show it
        if self.cursor_hidden {
            Self::temp_show_cursor();
        }

        print!("({}) > ", prompt_text);
        stdout().flush().expect("Error flushing stdOut!");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading stdIn!");

        // If we need to re-hide the cursor
        if self.cursor_hidden {
            Self::temp_hide_cursor();
        }

        // Trim player input and remove punctuation
        input.trim().replace(['.', '?', '!', ','], "")
    }
    /// Prompt player for input, then run `checker` with the input.
    ///
    /// Checker should return an `Option<ResultType>`.
    /// If checker's result is `Some(x)`, `prompt` will return x.
    ///
    /// Effectively, this lets you add a custom check function
    /// to determine if the input is valid or not.
    pub fn prompt<InputType, ResultType, Func>(
        &self,
        prompt_text: &str,
        error_msg: &str,
        checker: Func,
    ) -> ResultType
    where
        Func: Fn(&InputType) -> Option<ResultType>,
        InputType: FromStr + 'static,
    {
        loop {
            // Get raw input from the player and try to convert it into the correct type
            match self.prompt_raw(prompt_text).parse::<InputType>() {
                Ok(val) => {
                    // If it is convertible, run the checker function to see if it's valid
                    let check = checker(&val);
                    if let Some(result) = check {
                        // If it is valid, return the result
                        break result;
                    }
                }
                // If it can't be converted, error
                Err(_) => self.println(error_msg),
            }
        }
    }

    // Print a list of options, then let the user choose one, and return the chosen option
    pub fn options_prompt(&self, choices: Vec<&str>) -> u8 {
        // A new vec holding the options, so we can modify them
        let mut choices_list = Vec::<String>::new();
        // Add a number before each option (1., 2., etc)
        choices.iter().fold(1, |index, option| {
            // Push the number of the option & the option text to options
            choices_list.push(format!("{}. {}", index, option));

            // Increase the counter, so the next option's number is 1 higher
            index + 1
        });

        // Print the choices
        self.printlns(choices_list);

        // Prompt player for input, and return the number they chose
        self.prompt(
            "Choose 1",
            "Please type the number of the option you want.",
            |test_choice: &u8| {
                if usize::from(test_choice - 1) < choices.len() {
                    Some(*test_choice)
                } else {
                    None
                }
            },
        )
    }

    // Make the player choose one of two options
    pub fn two_option_prompt(&self, error_msg: &str, option_one: &str, option_two: &str) -> u8 {
        // Lowercase variants
        let option_two_lower = option_two.to_lowercase();
        let option_one_lower = option_one.to_lowercase();

        // Make a prompt for the player
        let prompt = format!("{option_one}/{option_two}");

        // Get player input, then checker will determine if it's option 1 or 2
        self.prompt(&prompt, error_msg, |choice: &String| {
            // Make the input lowercase
            let choice = choice.to_lowercase();

            if choice.chars().count() == 1 {
                // If the input is only 1 letter, see if it matches the first letter of either option
                let cropped_choice = choice.chars().next().unwrap();

                if cropped_choice == option_one_lower.chars().next().unwrap() {
                    Some(1)
                } else if cropped_choice == option_two_lower.chars().next().unwrap() {
                    Some(2)
                } else {
                    None
                }
            } else {
                // Otherwise, see if it matches either option
                if choice == option_one_lower {
                    Some(1)
                } else if choice == option_two_lower {
                    Some(2)
                } else {
                    None
                }
            }
        })
    }

    /// Yes/No Prompt: Returns true if the player responded "yes", and false if they responded "no".
    pub fn yes_no_prompt(&self) -> bool {
        self.two_option_prompt("Please type Y or N.", "Y", "n") == 1
    }

    /// Enable the AutoPrompt system
    pub fn autoprompt(&mut self) {
        self.autoprompt = true;
    }
}
