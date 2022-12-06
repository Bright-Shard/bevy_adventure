mod input;
mod keywords;
mod output;

use bevy::prelude::Resource;
use bevy::utils::HashMap;
use std::time::Duration;

/// Manage the game's input and output
/// IOManager is stored in a Bevy resource, so any system can use it.
// impls are in other files
#[derive(Resource)]
pub struct IOManager {
    pub(crate) cursor_hidden: bool,
    pub(crate) short_delay: Duration,
    pub(crate) medium_delay: Duration,
    pub(crate) long_delay: Duration,
    pub(crate) autoprompt: bool,
    pub(crate) punctuation: HashMap<char, Duration>,
}
impl IOManager {
    /// Make a new IOManager instance
    pub fn new() -> Self {
        let long_delay = Duration::from_millis(500);
        let medium_delay = Duration::from_millis(200);
        let short_delay = Duration::from_millis(50);

        let punctuation = HashMap::new();

        let mut this = Self {
            cursor_hidden: false,
            short_delay,
            medium_delay,
            long_delay,
            autoprompt: false,
            punctuation,
        };

        this.punctuation.insert(',', this.medium_delay);
        this.punctuation.insert(';', this.medium_delay);
        this.punctuation.insert('.', this.long_delay);
        this.punctuation.insert('?', this.long_delay);
        this.punctuation.insert('!', this.long_delay);
        this.punctuation.insert('\n', this.long_delay);

        this
    }
}
impl Default for IOManager {
    /// Make a new IOManager this with the Default trait
    fn default() -> Self {
        Self::new()
    }
}

// Re-exports
pub use keywords::{WordType, KEYWORDS};
