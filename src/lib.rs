// ========== DOCUMENTATION ==========

//! # Bevy Adventure
//! A Bevy plugin for making text-based adventure games.
//! For examples, see the examples directory on my GitHub repo.



// ========== MOD LOCAL FILES ==========
/// Input & Output manager struct
pub mod input_output_manager;
/// The systems built-in to bevy_adventure
pub mod systems;
/// Entity components built-in to bevy_adventure
pub mod components;
/// The events built-in to bevy_adventure & their traits
pub mod events;
/// The actual bevy_adventure plugin
pub mod plugin;
/// A trait to modify Bevy's Commands struct
pub mod adventure_commands;



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
