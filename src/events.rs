use bevy::prelude::{Component, Commands};
use crate::input_manager::WordType;
use bevy::ecs::schedule::IntoSystemDescriptor;
//use bevy_adventure_derive::Event;

// The basic handler type for events
pub type EventHandlerBase = fn(&Commands);
// Event handler for OnInteract
pub type InteractionHandler = fn(&mut Commands, WordType);

// EventHandler trait
pub trait EventHandler {}
impl EventHandler for EventHandlerBase {}

// Game events - they all store a handler, but have different types so that
//  they can be different components in ECS

// When an entity dies
#[derive(Component)]
pub struct OnDeath(pub EventHandlerBase);

// When an entity is interacted with
#[derive(Component)]
pub struct OnInteract(pub InteractionHandler);

// When an entity is looked at
#[derive(Component)]
pub struct OnView(pub EventHandlerBase);

// When a player enters a room
#[derive(Component)]
pub struct OnEnter(pub EventHandlerBase);

// Same as above, but it just prints text
#[derive(Component)]
pub struct OnEnterText(pub String);
