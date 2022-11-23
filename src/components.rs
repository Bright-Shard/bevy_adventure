use bevy::prelude::Component;

// ========== ENTITY COMPONENTS ==========

// Entity health
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

// For naming entities
#[derive(Component)]
pub struct Name(pub String);

// For giving entities descriptions
#[derive(Component)]
pub struct Description(pub String);

// ========== LEVEL COMPONENTS ==========

// A room in the game
#[derive(Component)]
pub struct Room {
    pub name: String,
    pub description: String
}

// The currently active room
#[derive(Component)]
pub struct ActiveRoom;

// A level in the game
#[derive(Component)]
pub struct Level;

// The currently active level
#[derive(Component)]
pub struct ActiveLevel;



// ========== EVENTS ==========
use crate::events::EventHandler;
use std::sync::{Arc, Mutex};
use bevy_adventure_derive::Event;

// The event handler each event stores
type Handler = Arc<Mutex<dyn EventHandler>>;

/// When an entity dies
#[derive(Component, Event)]
pub struct OnDeath(pub(crate) Handler);

/// When an entity is interacted with
#[derive(Component, Event)]
pub struct OnInteract(pub(crate) Handler);

/// When the player enters a room
#[derive(Component, Event)]
pub struct OnEnterRoom(pub(crate) Handler);
