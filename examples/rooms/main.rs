use bevy::prelude::*;
use bevy_adventure::prelude::*;

fn door_callback(mut commands: Commands, current_room: Query<&Room, With<ActiveRoom>>) {
    match current_room.single().name.as_str() {
        "Room 1" => commands.set_room("Room 2"),
        "Room 2" => commands.set_room("Room 1"),
        _ => panic!("Door opened from room that isn't Room 1 or Room 2"),
    }
}

fn build_level_one(mut commands: Commands) {
    // Doorway from Room 1 to Room 2
    let doorway = commands
        .spawn(Name("door".to_owned()))
        .bind_event::<OnInteract, _>(door_callback)
        .id();
    // Doorway from Room 2 to Room 1
    let doorway_two = doorway;

    // Room 1
    let initial_room = commands
        .spawn(Room {
            name: "Room 1".to_string(),
            description: "You're in room 1".to_string(),
        })
        .insert(ActiveRoom)
        .add_child(doorway)
        .id();

    // Room 2
    let room_two = commands
        .spawn(Room {
            name: "Room 2".to_string(),
            description: "You're in room 2".to_string(),
        })
        .add_child(doorway_two)
        .id();

    // Spawn a Level with rooms 1 & 2 inside
    commands
        .spawn(Level)
        .add_child(initial_room)
        .add_child(room_two);
}

fn main() {
    App::new()
        .add_plugin(AdventurePlugin)
        .add_startup_system(build_level_one)
        .run();
}
