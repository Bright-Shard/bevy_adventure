use bevy::prelude::*;
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
            let (entity, room, _) = 
                world.query::<(Entity, &Room, With<ActiveRoom>)>().single_mut(world);
            // If the active room is also the room we're making active, just return
            //  (Otherwise the OnRoom event would fire again)
            if room.name == new_room {
                return
            }
            // Remove ActiveRoom
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