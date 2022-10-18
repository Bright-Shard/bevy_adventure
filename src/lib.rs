// Local libraries
mod input_manager;
mod systems;

// Exports
pub mod components;
pub mod events;
pub mod level;

// Imports from local libraries
use systems::*;
use input_manager::InputManager;
use events::{EventHandler, Event};

// Other libraries
use hecs::World;

// Types
type GameSystem = fn(&mut Game);

// The main game
pub struct Game {
    pub world: World,
    pub systems: Vec<GameSystem>,
    pub input: InputManager
}
// Constructor
impl Game {
    pub fn new() -> Self {
        let mut game_systems = Vec::<GameSystem>::new();

        game_systems.push(system_handle_dead);

        Self {
            world: World::new(),
            systems: game_systems,
            input: InputManager::new()
        }
    }
}
// Methods
impl Game {
    pub fn add_system(&mut self, system: GameSystem) {
        self.systems.push(system);
    }
    pub fn start(&mut self) {
        loop {
            self.update();
        }
    }
    pub fn update(&mut self) {
        let game_systems = self.systems.clone();
        game_systems.iter().for_each(|system| system(self));
    }
}
