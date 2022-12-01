use std::sync::{Arc, Mutex};

use bevy::ecs::system::SystemState;
use bevy::prelude::{
    Added, App, CoreStage, Entity, IntoSystemDescriptor, Query, StartupStage, SystemSet, With,
    World,
};

use crate::{
    components::{ActiveRoom, Health, OnDeath, OnEnterRoom, OnInteract, Room},
    events::EventHandler,
    input_output_manager::{IOManager, ParseError},
};

// ========== ADD SYSTEMS TO APP ==========

pub fn append_systems(app: &mut App) {
    // Init events at startup
    app.add_startup_system_to_stage(StartupStage::PostStartup, init_events);
    // All the other systems
    app.add_system_set_to_stage(CoreStage::PreUpdate, build_system_set());
    // In development, add code-checking systems
    #[cfg(debug_assertions)]
    app.add_startup_system_set_to_stage(StartupStage::PostStartup, build_debug_system_set());
}

// ========== NORMAL SYSTEMS ==========

// The normal systems in bevy_adventure
fn build_system_set() -> SystemSet {
    SystemSet::new()
        // First, handle dead entities
        .with_system(handle_dead)
        // Then, trigger any new room events
        .with_system(new_room_event.after(handle_dead))
        // Finally, get user input
        .with_system(player_input.after(new_room_event))
}

// Clear dead entities, and run their OnDeath event if they have one
fn handle_dead(world: &mut World) {
    // Store dead entities
    let mut dead = Vec::<Entity>::new();
    // Query dead entities, optionally with OnDeath handlers
    let mut query = world.query::<(Entity, &Health, Option<&OnDeath>)>();
    // OnDeath events to handler
    let mut events: Vec<Arc<Mutex<dyn EventHandler>>> = Vec::new();

    // Iterate over query results
    for (entity, health, event) in query.iter(world) {
        // Entity has died
        if health.0 <= 0 {
            // Add it to the vec of dead entities
            dead.push(entity);
            // Check if the entity has an OnDeath event, if it does, run it
            if let Some(event) = event {
                events.push(event.0.clone());
            }
        }
    }

    // Fire any OnDeath events
    for event in events.iter_mut() {
        event.lock().unwrap().fire(world);
    }

    // Now run any on_death events

    // Now remove the dead entities
    dead.iter().for_each(|entity| {
        world.despawn(*entity);
    })
}

// When the player enters a new room that has an OnEnter
fn new_room_event(world: &mut World) {
    // Query for a room that just got ActiveRoom
    let mut query = SystemState::<Query<Option<&OnEnterRoom>, Added<ActiveRoom>>>::new(world);
    // See if there is a new room
    match query.get_mut(world).get_single() {
        // If not, do nothing
        Err(_) => {}
        // If there is a new room, fire the appropriate events (if they're registered)
        Ok(on_enter) => {
            // If it has an on_enter event handler, run the handler
            match on_enter {
                None => {}
                Some(event) => event.0.clone().lock().unwrap().fire(world),
            };
        }
    }
}

// Get player input & run any needed events
fn player_input(world: &mut World) {
    // This system's queries
    let mut active_room_query: SystemState<Query<&Room, With<ActiveRoom>>> =
        SystemState::new(world);

    // Get the active room
    let active_room = active_room_query.get(world);

    // Print the room's description
    IOManager::println(&active_room.single().description);

    // Prompt the player for input
    let prompt = IOManager::prompt("What do you do?", "Please type a valid action.");
    // Parse the player's input
    match prompt.parse(crate::input_output_manager::KEYWORDS, world) {
        Ok(target) => {
            // Get an EntityMut so we can see the target's components
            let target_mut = world.entity(target);

            // See if the action target has an event
            if let Some(event) = target_mut.get::<OnInteract>() {
                event.0.clone().lock().unwrap().fire(world);
            } else {
                // If it doesn't, print a generic confusion message
                IOManager::println("You can't do that with");
            }
        }
        // Print a generic confusion message if the prompt isn't understood
        Err(err) => match err {
            ParseError::NoAction => IOManager::println("I don't understand that action."),
            ParseError::NoTarget => IOManager::println("That's not an object"),
        },
    }
}

// ========== STARTUP SYSTEMS ==========

// Initialize all Events (SystemFunctions must be initialized before use)
fn init_events(world: &mut World) {
    // Queries for this system
    let mut state: SystemState<
        Query<(Option<&OnDeath>, Option<&OnInteract>, Option<&OnEnterRoom>)>,
    > = SystemState::new(world);

    // Events to init
    let mut events: Vec<Arc<Mutex<dyn EventHandler>>> = Vec::new();

    // Iterate over queried events, push them to events
    let queries = state.get(world);
    for (on_death, on_interact, on_enter_room) in queries.iter() {
        if let Some(event) = on_death {
            events.push(event.0.clone());
        }
        if let Some(event) = on_interact {
            events.push(event.0.clone());
        }
        if let Some(event) = on_enter_room {
            events.push(event.0.clone());
        }
    }

    // Iterate over events and init them (must be done this way because otherwise world is borrowed twice)
    for event in events.iter_mut() {
        event.lock().unwrap().init(world);
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
#[cfg(debug_assertions)]
fn active_room_check(query: Query<Option<&ActiveRoom>, With<Room>>) {
    // Count the number of rooms
    let mut active_rooms: i32 = 0;

    // Iterate through all rooms
    for active_room in &query {
        if active_room.is_some() {
            active_rooms += 1;
        }
    }

    // Make sure there's only 1 room
    if active_rooms == 0 {
        panic!(
            "\x1B[01;31mNo active room was set! \
                Remember to give one room the `ActiveRoom` component. \
                This is the room that the player will start in.\x1B[0m"
        );
    } else if active_rooms > 1 {
        panic!(
            "\x1B[1;31mToo many active rooms were set! \
                \x1B[0;31mOnly one room can have the `ActiveRoom` component. \
                This is the room that the player will start in.\x1B[0m"
        );
    }
}
