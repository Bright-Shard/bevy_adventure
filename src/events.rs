// Local imports
use crate::Game;

// Data for the event
pub trait EventData {}

// The Event trait
pub trait Event {
    fn fire(&self, game: &mut Game);
}

// Handlers for events
pub type EventHandler = fn(&mut Game);

// Game events - these are structs, so they can be used as components in hecs
pub struct OnDeath(EventHandler);
impl Event for OnDeath {
    fn fire(&self, game: &mut Game) {
        self.0(game);
    }
}
pub struct OnInteract(EventHandler);
