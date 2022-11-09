use crate::InputManager;
use std::{str::FromStr, io::{stdout, stdin, Write}};
use core::any::TypeId;

use bevy::prelude::{Query, Without, Commands};

use crate::input_manager::keywords::WordType;
use crate::components::{ActiveRoom, Name};
use crate::events::{OnInteract, OnView};

// Errors when parsing input
pub enum ParseError {
    NoRegisteredEvent,
    NoTarget,
    NoAction
}

// Target of player input
struct Target <'a> {
    on_view: Option<&'a OnView>,
    on_interact: Option<&'a OnInteract>
}
impl <'a> Target <'a> {
    pub fn view(&self, commands: Commands) -> Result<(), ParseError> {
        match self.on_view {
            None => Err(ParseError::NoRegisteredEvent),
            Some(view_event) => {
                view_event.0(&commands);
                Ok(())
            }
        }
    }
    pub fn interact(&self, mut commands: Commands, keyword: WordType) -> Result<(), ParseError> {
        match self.on_interact {
            None => Err(ParseError::NoRegisteredEvent),
            Some(interaction_event) => {
                interaction_event.0(&mut commands, keyword);
                Ok(())
            }
        }
    }
}

// The returned user input
pub struct Input <T: FromStr> {
    value: T,
    error_msg: String
}
impl <T: FromStr> Input <T> {
    pub fn new(value: T, error_msg: &str) -> Self {
        Self {
            value,
            error_msg: error_msg.to_string()
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn get_owned(self) -> T {
        self.value
    }

    pub fn error(&self) {
        println!("{}", self.error_msg);
    }
}
impl Input <String> {
    pub fn parse(
        &self,
        keywords: phf::Map<&'static str, WordType>,
        named_entities: Query<(&Name, Option<&OnView>, Option<&OnInteract>), Without<ActiveRoom>>,
        commands: Commands
    ) -> Result<(), ParseError> {
        let split = self.value.split_whitespace();

        let mut potential_targets = Vec::new();
        let mut action = WordType::Ignore;
        let mut target: Option<Target> = None;

        split.for_each(|word| {
            match keywords.get(&word.to_lowercase()) {
                None => potential_targets.push(word),
                Some(word_type) => match word_type {
                    WordType::Ignore => {},
                    _ => action = word_type.clone()
                }
            }
        });

        for potential_target in potential_targets {
            for (name, on_view, on_interact) in &named_entities {
                if name.0 == potential_target {
                    target = Some(Target {
                        on_interact,
                        on_view
                    });
                }
            }
        }

        match target {
            None => Err(ParseError::NoTarget),
            Some(target) => {
                match action {
                    WordType::Ignore => Err(ParseError::NoAction),
                    WordType::Look => target.view(commands),
                    keyword => target.interact(commands, keyword)
                }
            }
        }
    }
}
impl Input <i32> {
}

// Input from player
impl InputManager {
    pub fn prompt <T: FromStr + 'static> (prompt: &str, error_msg: &str) -> Input<T> {
        // Prompt the player for input, as long as the prompt isn't blank
        if prompt != "" {
            Self::println(prompt);
        }

        // Now make the prompt for the player to type into
        let prompt_type: &str;
        // If the prompt type is a string, set prompt_type to text
        // (prompt_type will be used to build the prompt)
        if TypeId::of::<T>() == TypeId::of::<String>() {
            prompt_type = "Text";
        // If it's an i32, set prompt_type to number
        } else if TypeId::of::<T>() == TypeId::of::<i32>() {
            prompt_type = "Number";
        } else {
            // Failsafe
            panic!("Unknown type to get input for!")
        }
        // The actual prompt
        let prompt = format!("({prompt_type}) > ");

        // Now loop until the player types something that can be converted into the type we need
        loop {
            // Make a prompt
            print!("{}", prompt);
            stdout().flush().expect("Error flushing stdout!");
        
            // For storing the player input
            let mut input = String::new();

            // Get the input they type
            stdin().read_line(&mut input).unwrap();

            // Match it to make sure it's the correct type
            match input.trim().parse::<T>() {
                // If it is, return an Input of it
                Ok(value) => break Input::new(value, error_msg),
                Err(_) => Self::println(error_msg)
            }
        }
    }

    // Print a list of options, then let the user choose one, and return the chosen option
    pub fn options_prompt(prompt: &str, raw_options: Vec<&str>) -> String {
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
        Self::printlns(options);

        // Loop getting player input until they choose a valid option, then return it
        loop {
            // Get player input
            let input = Self::prompt::<i32>(
                "Choose 1",
                "Please type the number of the option you want."
            );

            // Now make sure the input is actually one of the options we printed
            match raw_options.get((input.get() - 1) as usize) {
                Some(choice) => break String::from(*choice),
                None => Self::println("Please choose an option that was printed.")
            }
        }
    }
}