use bevy::prelude::{Entity, Commands, With, World};
use crate::components::{Room, ActiveRoom};

// Add commands to Bevy's Commands struct
pub trait AdventureCommands {
    // Change the active room
    fn set_room(&mut self, room: String);
}
impl AdventureCommands for Commands<'_, '_> {
    fn set_room(&mut self, new_room: String) {
        self.add(move |world: &mut World| {
            // Remove ActiveRoom for the current room
            let (entity, _, _) = 
                world.query::<(Entity, With<Room>, With<ActiveRoom>)>().single_mut(world);
            world.get_entity_mut(entity).unwrap().remove::<ActiveRoom>();

            // Add ActiveRoom for the newly active room
            let mut query = world.query::<(
                Entity,
                &Room
            )>();
            let mut target = None;
            for (entity, room) in query.iter_mut(world) {
                if room.name == new_room {
                    target = Some(entity);
                }
            }
            match target {
                Some(room) => world.entity_mut(room).insert(ActiveRoom{}),
                None => panic!("Attempted to set the active room to `{}`, which does not exist", new_room)
            };
        });
    }
}