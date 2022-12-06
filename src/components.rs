use bevy::prelude::Component;

// ========== ENTITY COMPONENTS ==========

/// Entity health component
///
/// bevy_adventure will check this component to
/// see if any entities have died.
#[derive(Component)]
pub struct Health(pub i32);

impl Health {
    pub fn damage(&mut self, damage: i32) {
        self.0 -= damage;
    }
    pub fn heal(&mut self, amount: i32) {
        self.0 += amount;
    }
    pub fn set_health(&mut self, health: i32) {
        self.0 = health;
    }
}

/// For naming entities
#[derive(Component)]
pub struct Name(pub &'static str);

/// Alias an entity
///
/// The player can then interact with the entity
/// by using its alias, in addition to its name.
#[derive(Component)]
pub struct Aliases(pub Vec<&'static str>);

/// For giving entities descriptions
#[derive(Component)]
pub struct Description(pub String);

// ========== LEVEL COMPONENTS ==========

/// A room in the game
#[derive(Component)]
pub struct Room {
    /// Name of the room
    pub name: &'static str,
    /// Description of the room (printed w/ autoprompt)
    pub description: Option<&'static str>,
}

/// A marker component for the currently active room
#[derive(Component)]
pub struct ActiveRoom;

/// A level in the game
#[derive(Component)]
pub struct Level;

// ========== EVENTS ==========
use crate::{events::EventHandler, input_output_manager::WordType};
use std::sync::{Arc, Mutex};

// The event handler each event stores
type Handler = Arc<Mutex<dyn EventHandler>>;

/// When an entity dies
#[derive(Component)]
pub struct OnDeath(pub(crate) Handler);

/// When an entity is interacted with
///
/// This event takes one generic - the type of word this interaction handles.
#[derive(Component)]
pub struct OnInteract(pub(crate) bevy::utils::HashMap<WordType, Handler>);

/// When the player enters a room
#[derive(Component)]
pub struct OnEnterRoom(pub(crate) Handler);
