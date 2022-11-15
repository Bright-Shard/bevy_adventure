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

