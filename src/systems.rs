use std::borrow::Borrow;

// Libraries
use hecs::{Entity, EntityRef};
// Local imports
use crate::{Game, components::*, events::*};

// Clear dead entities, and run their OnDeath even if they have one
pub fn system_handle_dead(game: &mut Game) {
    // Entities that have died
    let dead = game.world.query::<&Health>()
        // Iterate through entities with health
        .iter()
        // Collect the ones that are dead
        .fold(Vec::<Entity>::new(), |mut dead, (entity, health)| {
            // If the entity has died
            if health.0 <= 0 {
                // Add entity to list of dead entites
                dead.push(entity);
            }

            // Return it for the next iteration
            return dead;
        });
    
    // Iterate through dead entities
    dead.iter().for_each(|entity| {
        // Remove the entity from the world & take it's components
        match game.world.take(*entity) {
            Ok(components) => {
                components.t
            },
            Err(_) => {}
        }
    });
}
