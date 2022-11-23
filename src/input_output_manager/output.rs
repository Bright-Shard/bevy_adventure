use super::IOManager;
use std::{thread, time, fmt::Display, process::Command, io::{stdout, Write}};

const DEFAULT_DELAY: u64 = 35;

// Output to player
impl IOManager {
    // For printing one line with no newline at the end
    pub fn print <F: ToString> (text: F) {
        // Print each character one at a time
        text.to_string().chars().for_each(|char| {
            print!("{}", char);
            // Force stdout to refresh
            stdout().flush().expect("Error flushing stdout!");
            // Delay between each character
            thread::sleep(time::Duration::from_millis(DEFAULT_DELAY));
        });
    }
    // For printing one line with a newline at the end
    pub fn println <F: ToString> (text: F) {
        Self::print(text);
        println!("");
    }
    // For printing multiple line
    pub fn printlns <F: Display> (text: Vec<F>) {
        // Print each line one at a time
        text.into_iter().for_each(|line| {
            // Print the line
            println!("{}", line);
            // Force stdout to refresh
            stdout().flush().expect("Error flushing stdout!");
            // Delay between each line
            thread::sleep(time::Duration::from_millis(DEFAULT_DELAY) * 4);
        })
    }
    // For clearing output (OS-Specific)
    #[cfg(target_family = "windows")]
    pub fn clear() {
        Command::new("cls")
            .spawn()
            .expect("Error clearing the screen.");
    }
    #[cfg(target_family = "unix")]
    pub fn clear() {
        Command::new("clear")
            .spawn()
            .expect("Error clearing the screen.");
    }
}
