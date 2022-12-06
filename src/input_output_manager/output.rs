use super::IOManager;
use std::{
    fmt::Display,
    io::{stdout, Write},
    thread,
};

fn flush() {
    stdout().flush().expect("Error flushing stdout!");
}

// Output to player
impl IOManager {
    /// For printing text with no newline at the end
    pub fn print<F: ToString>(&self, text: F) {
        // Print each character one at a time
        text.to_string().chars().for_each(|char| {
            print!("{}", char);
            // Force stdout to refresh
            flush();
            // Delay between each character
            if let Some(delay) = self.punctuation.get(&char) {
                // Bigger delay for punctuation
                thread::sleep(*delay)
            } else {
                // Normal delay for generic characters
                thread::sleep(self.short_delay);
            }
        });
    }
    /// For printing text with a newline at the end
    pub fn println<F: ToString>(&self, text: F) {
        self.print(text);
        println!();
    }
    /// For printing multiple lines of text one after another (intended for lists)
    pub fn printlns<F: Display>(&self, text: Vec<F>) {
        // Print each line one at a time
        text.into_iter().for_each(|line| {
            // Print the line
            println!("{}", line);
            // Force stdout to refresh
            flush();
            // Delay between each line
            thread::sleep(self.long_delay);
        })
    }
    /// For clearing output - uses ANSI escape codes to clear the terminal window
    pub fn clear(&self) {
        print!("\x1B[2J");
    }
    /// Temporarily hide the cursor (doesn't update `self.cursor_hidden`)
    pub fn temp_hide_cursor() {
        print!("\x1B[?25l");
    }
    /// Temporarily show the cursor (doesn't update `self.cursor_hidden`)
    pub fn temp_show_cursor() {
        print!("\x1B[?25h");
    }
    /// Uses ANSI escape codes to hide the terminal cursor
    pub fn hide_cursor(&mut self) {
        Self::temp_hide_cursor();

        // Update hide_cursor
        self.cursor_hidden = true;
    }
    /// Uses ANSI escape codes to show the terminal cursor
    pub fn show_cursor(&mut self) {
        Self::temp_show_cursor();

        // Update hide_cursor
        self.cursor_hidden = false;
    }
}
