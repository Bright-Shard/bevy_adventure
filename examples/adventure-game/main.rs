use bevy::prelude::*;
use bevy_adventure::prelude::*;
type Name = bevy_adventure::prelude::Name;

fn door_callback(mut commands: Commands, names: Query<&Name>) {
    commands.set_room("Room 2".to_string());
    commands.add(|world: &mut World| {
        world.spawn(Name("ItWorkedXD".to_string()));
        println!("Exclusive access to world!");
    });
    for name in names.iter() {
        println!("Name: {}", name.0);
    }
    println!("EEE");
}

fn build_level_one(mut commands: Commands) {
    // Random entity in Room 2
    let example = commands.spawn(
        Name("EXZAMPLEFSD".to_owned())
    ).id();
    // Doorway from Room 1 to Room 2
    let doorway = commands.spawn(
        (
            Name("door".to_owned()),
            OnInteract::new(IntoSystem::into_system(door_callback))
        )
    )
        .id();

    // Room 1
    let initial_room = commands.spawn(
        Room {
            name: "Room 1".to_string(),
            description: "You're in room 1".to_string()
        }
    )
        .insert(ActiveRoom)
        .add_child(doorway)
        .id();
    
    // Room 2
    let room_two = commands.spawn(
        Room {
            name: "Room 2".to_string(),
            description: "You're in room 2".to_string()
        }
    )
        .add_child(example)
        .id();
    
    // Spawn a Level with rooms 1 & 2 inside
    commands.spawn(Level)
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
