use super::IOManager;
use std::{str::FromStr, io::{stdout, stdin, Write}};
use core::any::TypeId;

use bevy::{prelude::{Query, Entity, Children, HierarchyQueryExt, World, With}, ecs::system::SystemState};

use crate::input_output_manager::keywords::WordType;
use crate::components::{Name, ActiveRoom};

// Errors when parsing input
pub enum ParseError {
    NoRegisteredEvent,
    NoTarget,
    NoAction
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
    // Gets the target and action, then runs appropriate handlers
    pub fn parse_action(&self, keywords: phf::Map<&'static str, WordType>, world: &mut World) -> Result<Entity, ParseError> {

        // This system's queries
        let mut state: SystemState<(
            Query<&Children>,
            Query<Entity, With<ActiveRoom>>,
            Query<&Name>
        )> = SystemState::new(world);
        let (children, active_room, names) = state.get(world);

        let split = self.value.split_whitespace();

        let mut potential_targets = Vec::new();
        let mut action = WordType::Ignore;

        // Iterate through words and see if they are keywords or not
        split.for_each(|word| {
            match keywords.get(&word.to_lowercase()) {
                // If it isn't a keyword, it might be a target
                None => potential_targets.push(word),
                // If it is a keyword,
                Some(word_type) => match word_type {
                    // Either ignore it
                    WordType::Ignore => {},
                    // Or set it as the action.
                    _ => action = word_type.clone()
                }
            }
        });

        // If an action type wasn't identified, error out
        if let WordType::Ignore = action {
            return Err(ParseError::NoAction);
        }

        // Try and get the target of the action
        // Iterate through the active children
        for child in children.iter_descendants(active_room.get_single().unwrap()) {
            // See if the child has a name
            match names.get(child) {
                Ok(name) => {
                    // If it does, see if that name is in potential_targets
                    if potential_targets.iter().any(|test_name| *test_name == &name.0) {
                        // We've found the target!
                        return Ok(child);
                    }
                },
                Err(_) => {}
            }
        }

        // If we haven't identified the target already, error that it wasn't found
        Err(ParseError::NoTarget)
    }
}
impl Input <i32> {
}

// Input from player
impl IOManager {
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