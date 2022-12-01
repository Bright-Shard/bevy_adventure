// ========== DOCUMENTATION ==========

//! # Bevy Adventure
//! A Bevy plugin for making text-based adventure games.
//! For examples, see the
//! [examples directory](https://github.com/Bright-Shard/bevy_adventure/tree/master/examples)
//! on my GitHub repo.

// ========== MOD LOCAL FILES ==========
/// Add methods to Bevy's [Commands] and [EntityCommands] structs
pub mod adventure_commands;
/// Entity components built-in to bevy_adventure
pub mod components;
/// The events built-in to bevy_adventure & their traits
pub mod events;
/// Input & Output manager struct
pub mod input_output_manager;
/// The actual bevy_adventure plugin
pub mod plugin;
/// The systems built-in to bevy_adventure
pub mod systems;

// ========== Exports/Modules ==========

/// Everything needed to use bevy_adventure. Import this when using the library.
pub mod prelude {
    // Input manager
    pub use crate::input_output_manager::{IOManager, WordType, KEYWORDS};
    // Built in components
    pub use crate::components::*;
    // Trait modifying Bevy's Commands struct
    pub use crate::adventure_commands::{AdventureCommands, AdventureEntityCommands};
    // The plugin itself
    pub use crate::plugin::AdventurePlugin;
    // Event trait so `Event::new()` can be used
    pub use crate::events::Event;
}
