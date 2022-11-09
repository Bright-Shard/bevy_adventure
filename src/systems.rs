// Libraries
use bevy::prelude::{Query, Entity, Commands, With, Added, SystemSet, Without, ParallelSystemDescriptorCoercion};
// Local imports
use crate::{components::*, events::*, InputManager};

pub fn build_system_set() -> SystemSet {
    return SystemSet::new()
        .with_system(new_room_event)
        .with_system(handle_active_room.after(new_room_event))
        .with_system(handle_dead)
}

// Clear dead entities, and run their OnDeath even if they have one
fn handle_dead(mut commands: Commands, query: Query<(Entity, &Health, Option<&OnDeath>)>) {
    // Store dead entities
    let mut dead = Vec::<Entity>::new();

    // Iterate over query results
    for (entity, health, death_handler) in &query {
        // Entity has died
        if health.0 <= 0 {
            // Add it to the vec of dead entities
            dead.push(entity);

            // Check if the entity has a death handler
            match death_handler {
                None => {},
                // If it does, run it
                Some(handler) => handler.0(&commands)
            }
        }
    }

    // Now remove the dead entities
    dead.iter().for_each(|entity| {
        commands.entity(*entity).despawn();
    })
}

// When the player enters a new room that has an OnEnter
fn new_room_event(commands: Commands, query: Query<(Option<&OnEnter>, Option<&OnEnterText>), Added<ActiveRoom>>) {
    // See if there is a new room
    match query.get_single() {
        // If not, do nothing
        Err(_) => {},
        // If there is a new room, fire the appropriate events (if they're registered)
        Ok((on_enter, on_enter_text)) => {
            // If it has an on_enter event handler, run the handler
            match on_enter {
                None => {},
                Some(handler) => handler.0(&commands)
            };
            // If it has text, print that text
            match on_enter_text {
                None => {},
                Some(text) => InputManager::println(&text.0)
            };
        }
    }
}

// Manage the room the player is in
fn handle_active_room(
    query: Query<&Room, With<ActiveRoom>>,
    named_entities: Query<(&Name, Option<&OnView>, Option<&OnInteract>), Without<ActiveRoom>>,
    commands: Commands
)
{
    // Get the active room
    let room = query.single();
    InputManager::println(&room.description);
    let prompt = InputManager::prompt("What do you do?", "Please type a valid action.");
    match prompt.parse(crate::input_manager::KEYWORDS, named_entities, commands) {
            Ok(_) => {},
            Err(_) => prompt.error()
    }
}



// ========== CODE CHECKING SYSTEMS ==========
// In development, add some code checks to the app start
#[cfg(debug_assertions)]
pub fn build_debug_system_set() -> SystemSet {
    SystemSet::new()
        //.with_system(room_checks)
        .with_system(active_room_check)
}

// Make sure an active room was defined, and make sure only 1 was defined
fn active_room_check(query: Query<Option<&ActiveRoom>, With<Room>>) {
    // Count the number of rooms
    let mut active_rooms:i32 = 0;

    // Iterate through all rooms
    for active_room in &query {
        match active_room {
            Some(_) => active_rooms += 1,
            None => {}
        }
    }

    // Make sure there's only 1 room
    if active_rooms == 0 {
        panic!("\x1B[01;31mNo active room was set! \
                Remember to give one room the `ActiveRoom` component. \
                This is the room that the player will start in.\x1B[0m");
    } else if active_rooms > 1 {
        panic!("\x1B[1;31mToo many active rooms were set! \
                \x1B[0;31mOnly one room can have the `ActiveRoom` component. \
                This is the room that the player will start in.\x1B[0m");
    }
}
