use bevy::prelude::*;
use bevy_adventure::prelude::*;

fn build_level_one(mut commands: Commands) {
    let doorway = commands.spawn()
        .insert(OnInteract(|commands, interaction_type| {
            commands.set_room("Room 2".to_string());
            println!("Moved to Room 2 because of interaction {:?}", interaction_type)
        }))
        .insert(Name("door".to_owned()))
        .id();

    let initial_room = commands.spawn()
        .insert(Room::new("Room 1", "You're in room 1"))
        .insert(OnEnterText("You wake up in a cold, dark room.".to_string()))
        .insert(ActiveRoom)
        .add_child(doorway)
        .id();
    
    let room_two = commands.spawn()
        .insert(Room::new("Room 2", "You're in room 2"))
        .insert(OnEnterText(
            "Compared to the last room, this room is much warmer and brighter.".to_string())
        )
        .id();
    
    commands.spawn()
        .insert(Level)
        .add_child(initial_room)
        .add_child(room_two)
    ;
}

fn main() {
    App::new()
        .add_plugin(AdventurePlugin)
        .add_startup_system(build_level_one)
        .run();
}
