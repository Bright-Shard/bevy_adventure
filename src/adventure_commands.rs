use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Commands, World, Entity, With};
use crate::components::{Room, ActiveRoom};

/// Add commands to Bevy's Commands struct
pub trait AdventureCommands {
    /// Change the active room in the game
    fn set_room(&mut self, room: &str);
}
impl AdventureCommands for Commands<'_, '_> {
    fn set_room(&mut self, new_room: &str) {
        // Get the room name
        let room_name = new_room.to_owned();

        self.add(move |world: &mut World| {
            // Remove ActiveRoom for the current room
            let (entity, room, _) = 
                world.query::<(Entity, &Room, With<ActiveRoom>)>().single_mut(world);
            // If the active room is also the room we're making active, just return
            //  (Otherwise the OnRoom event would fire again)
            if room.name == room_name {
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
                if room.name == room_name {
                    target = Some(entity);
                }
            }
            match target {
                Some(room) => world.entity_mut(room).insert(ActiveRoom{}),
                None => panic!("Attempted to set the active room to `{}`, which does not exist", room_name)
            };
        });
    }
}

/// Modify Bevy's EntityCommands struct
use crate::events::{Event, IntoEventHandler};
use bevy::prelude::Component;

pub trait AdventureEntityCommands {
    fn bind_event<EventType: Event + Component, Params>(&mut self, handler: impl IntoEventHandler<Params>) -> &mut Self;
}
impl AdventureEntityCommands for EntityCommands<'_, '_, '_> {
    fn bind_event<EventType: Event + Component, Params>(&mut self, handler: impl IntoEventHandler<Params>) -> &mut Self {
        self.insert(
            EventType::new(handler.into_event())
        )
    }
}
