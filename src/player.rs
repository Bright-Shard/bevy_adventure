use bevy::prelude::{Entity, Resource};

pub trait InventoryItem: Sync + Send {}

#[derive(Resource)]
pub struct Inventory(pub Vec<Entity>);

#[derive(Resource)]
pub struct Checkpoints(pub Vec<&'static str>);
