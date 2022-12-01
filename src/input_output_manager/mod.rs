mod input;
mod keywords;
mod output;

/// Manage the game's input and output
// impls are in other files
pub struct IOManager;

// Re-exports
pub use input::ParseError;
pub use keywords::{WordType, KEYWORDS};
