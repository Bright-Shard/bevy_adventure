// Local files
mod input_manager;
mod systems;
mod components;
mod events;
mod adventure_commands;

// Exports
pub mod prelude {
    pub use crate::input_manager::{InputManager, WordType};
    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::adventure_commands::AdventureCommands;
    pub use crate::AdventurePlugin;
}
pub use input_manager::InputManager;
pub use components::*;
pub use events::*;

// Imports from local files

// Libraries
use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

// Additional plugins for an adventure game
// Core ones are added by the MinimalPlugins plugins group
struct AdventureDefaultPlugins;
impl PluginGroup for AdventureDefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        // Hierarchy for objects in the game
        group.add(bevy::hierarchy::HierarchyPlugin::default());
        // Player input
        group.add(bevy::input::InputPlugin::default());

        /*
        // Windowing
        group.add(bevy::window::WindowPlugin::default());
        group.add(bevy::winit::WinitPlugin::default());
        // UI & Text
        group.add(bevy::ui::UiPlugin::default());
        group.add(bevy::text::TextPlugin::default());
        */
    }
}

// The Bevy plugin
pub struct AdventurePlugin;
impl Plugin for AdventurePlugin {
    fn build (&self, app: &mut App) {
        // In development, add a code-checking system
        #[cfg(debug_assertions)]
        app.add_startup_system_set_to_stage(
            StartupStage::PostStartup,
            systems::build_debug_system_set()
        );
        
        // All normal systems & plugins
        app
            // Core Bevy plugins
            .add_plugins(MinimalPlugins)
            // Other plugins needed for adventure games
            .add_plugins(AdventureDefaultPlugins)
            // The systems for adventure games
            .add_system_set(systems::build_system_set());
    }
}
