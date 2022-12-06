use std::sync::{Arc, Mutex};

use bevy::ecs::system::SystemState;
use bevy::prelude::{
    Added, App, Children, CoreStage, Entity, EventReader, HierarchyQueryExt, IntoSystemDescriptor,
    Mut, Query, Res, ResMut, Resource, StartupStage, SystemSet, With, World,
};

use crate::components::{Aliases, Name};
use crate::{
    components::{ActiveRoom, Health, OnDeath, OnEnterRoom, OnInteract, Room},
    events::EventHandler,
    input_output_manager::{IOManager, WordType, KEYWORDS},
};

// ========== ADD SYSTEMS TO APP ==========

/// Appends all of bevy_adventure's systems to the app
pub fn append_systems(app: &mut App) {
    // Init events at startup
    app.add_startup_system_to_stage(StartupStage::PostStartup, init_events);
    // A generic system listening for Bevy's exit
    app.add_system(on_exit);
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

    // Now remove the dead entities
    dead.iter().for_each(|entity| {
        world.despawn(*entity);
    })
}

// SystemState caching for new_room_event
type NewRoomQuery<'a> = Query<'static, 'static, Option<&'a OnEnterRoom>, Added<ActiveRoom>>;
#[derive(Resource)]
pub struct NewRoomState(SystemState<(NewRoomQuery<'static>, ResMut<'static, IOManager>)>);
impl NewRoomState {
    pub fn new(world: &mut World) -> Self {
        Self(SystemState::new(world))
    }
}

// When the player enters a new room that has an OnEnterRoom handler
fn new_room_event(world: &mut World) {
    // Get our cached SystemState
    world.resource_scope(|world, mut state: Mut<NewRoomState>| {
        // Get our query
        let (query, mut iomgr) = state.0.get_mut(world);
        // Event, if it exists
        let mut event = None;

        // See if there is a new room
        match query.get_single() {
            // If not, do nothing
            Err(_) => {}
            // If there is a new room, fire the appropriate events (if they're registered)
            Ok(on_enter) => {
                // If it has an on_enter event handler, run the handler
                if let Some(handler) = on_enter {
                    event = Some(handler.0.clone());
                }
                // Also disable AutoPrompt so it's only on if set manually
                iomgr.autoprompt = false;
            }
        }

        // If there was an event handler, fire it
        if let Some(handler) = event {
            handler.lock().unwrap().fire(world);
        }
    });
}

// An event listener, so when bevy exits we can re-enable the terminal cursor
fn on_exit(exit: EventReader<bevy::app::AppExit>, mut iomgr: ResMut<IOManager>) {
    if !exit.is_empty() {
        iomgr.show_cursor();
    }
}

// Player autoprompt
fn player_input(world: &mut World) {
    // Query types
    type ActiveRoomQuery<'world, 'state, 'a> =
        Query<'world, 'state, (Entity, &'a Room), With<ActiveRoom>>;
    type ChildrenQuery<'world, 'state, 'a> = Query<'world, 'state, &'a Children>;
    type NameQuery<'world, 'state, 'a> = Query<'world, 'state, (&'a Name, Option<&'a Aliases>)>;
    type IOManagerResource<'w> = Res<'w, IOManager>;

    // This system's queries
    let mut active_room_query: SystemState<(
        ActiveRoomQuery,
        ChildrenQuery,
        NameQuery,
        IOManagerResource,
    )> = SystemState::new(world);

    // Get queries
    let (active_room_query, children, names, iomgr) = active_room_query.get(world);

    // If AutoPrompt is disabled, return
    if !iomgr.autoprompt {
        return;
    }

    let (active_room_entity, active_room) = active_room_query.single();

    // Print the room's description
    if let Some(desc) = &active_room.description {
        iomgr.println(desc);
    }

    // Prompt the player for input
    let input = iomgr.prompt_raw("What do you do?");

    // Parse the input
    let split = input.split_whitespace();

    let mut potential_targets = Vec::new();
    let mut action: Option<WordType> = None;

    // Iterate through words and see if they are keywords or not
    split.for_each(|word| {
        match KEYWORDS.get(&word.to_lowercase()) {
            // If it isn't a keyword, it might be a target
            None => potential_targets.push(word),
            // If it is a keyword,
            Some(word_type) => match word_type {
                // Either ignore it
                WordType::Ignore => {}
                // Or set it as the action.
                _ => {
                    if action.is_none() {
                        action = Some(word_type.clone());
                    }
                }
            },
        }
    });

    // Try and get the target of the action
    // Iterate through the active children
    let mut target = children.iter_descendants(active_room_entity).find(|child| {
        if let Ok((name, aliases)) = names.get(*child) {
            // Iterate through potential targets
            potential_targets
                .iter()
                // See if any of the potential targets match this entity's name
                .any(|test_name| {
                    // Also see if the entity has aliases to check
                    if let Some(alias_list) = aliases {
                        (alias_list.0.contains(test_name)) || (*test_name == name.0)
                    } else {
                        // If not, just check the entity name
                        *test_name == name.0
                    }
                })
        } else {
            false
        }
    });

    // If we haven't identified the target already, fall back to the room
    if target.is_none() {
        target = Some(active_room_entity);
    }

    // Act upon the player's input
    // Get an EntityMut so we can see the target's components
    let target_mut = world.entity(target.unwrap());

    // See if the action target has an event
    if let Some(event) = target_mut.get::<OnInteract>() {
        // If it does, see if an action was identified
        if let Some(action_type) = action {
            // If we have an OnInteract even and action, make sure their types match
            if let Some(handler) = event.0.get(&action_type) {
                handler.clone().lock().unwrap().fire(world);
            // WordType::Any is always a fallback
            } else if let Some(handler) = event.0.get(&WordType::Any) {
                handler.clone().lock().unwrap().fire(world);
            }
        } else {
            // If an action wasn't identified, see if the target has a handler for WordType::Any
            if let Some(handler) = event.0.get(&WordType::Any) {
                handler.clone().lock().unwrap().fire(world);
            } else {
                // If there isn't one, just error out
                iomgr.println("I don't understand that action.");
            }
        }
    } else {
        // If it doesn't, print a generic confusion message
        iomgr.println("Sorry, that action is confusing.");
    }
}

// ========== STARTUP SYSTEMS ==========

// Initialize all Events (SystemFunctions must be initialized before use)
fn init_events(world: &mut World) {
    // Query types
    type OnDeathHandler<'a> = Option<&'a OnDeath>;
    type OnInteractionHandler<'a> = Option<&'a OnInteract>;
    type OnEnterRoomHandler<'a> = Option<&'a OnEnterRoom>;

    // Queries for this system
    let mut state: SystemState<Query<(OnDeathHandler, OnInteractionHandler, OnEnterRoomHandler)>> =
        SystemState::new(world);

    // Events to init
    let mut events: Vec<Arc<Mutex<dyn EventHandler>>> = Vec::new();

    // Iterate over queried events, push them to events
    let queries = state.get(world);
    for (on_death, on_interact, on_enter_room) in queries.iter() {
        if let Some(event) = on_death {
            events.push(event.0.clone());
        }
        if let Some(event) = on_interact {
            for handler in event.0.values() {
                events.push(handler.clone());
            }
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
fn build_debug_system_set() -> SystemSet {
    SystemSet::new().with_system(active_room_check)
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
