use bevy::prelude::{MinimalPlugins};
use bevy::app::{PluginGroupBuilder, PluginGroup, Plugin, App};
use crate::systems;

/// Plugins needed by bevy_adventure.
struct AdventureDefaultPlugins;
impl PluginGroup for AdventureDefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // Hierarchy for objects in the game
            .add(bevy::hierarchy::HierarchyPlugin::default())
            // Player input
            .add(bevy::input::InputPlugin::default())
    }

    fn name() -> String {
        return String::from("BevyAdventurePlugins")
    }
}

/// The bevy_adventure plugin itself. This adds [AdventureDefaultPlugins] and [MinimalPlugins] to the app.
pub struct AdventurePlugin;
impl Plugin for AdventurePlugin {
    fn build (&self, app: &mut App) {
        // Add plugins
        app
            // Core Bevy plugins
            .add_plugins(MinimalPlugins)
            // Other plugins needed for adventure games
            .add_plugins(AdventureDefaultPlugins);
        
        // Add systems
        systems::append_systems(app);
    }
}