use crate::components::{ActiveRoom, Room};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{Commands, Entity, With, World};

/// Add methods to Bevy's [Commands] struct
pub trait AdventureCommands {
    /// Change the active room in the game
    ///
    /// Just pass the name of the room to make active.
    /// This should match the string in that room's `name` field.
    ///
    /// Check the
    /// [rooms example](https://github.com/Bright-Shard/bevy_adventure/tree/master/examples/rooms)
    /// for an example on how to use this.
    fn set_room(&mut self, room: &str);
}
impl AdventureCommands for Commands<'_, '_> {
    fn set_room(&mut self, new_room: &str) {
        // Get the room name
        let room_name = new_room.to_owned();

        // Add a command to the queue, so we get exclusive world access
        self.add(move |world: &mut World| {
            // Remove ActiveRoom for the current room
            let (entity, room, _) = world
                .query::<(Entity, &Room, With<ActiveRoom>)>()
                .single_mut(world);
            // If the active room is also the room we're making active, just return
            //  (Otherwise the OnRoom event would fire again)
            if room.name == room_name {
                return;
            }
            // Remove ActiveRoom
            world.get_entity_mut(entity).unwrap().remove::<ActiveRoom>();

            // Add ActiveRoom for the newly active room
            let mut query = world.query::<(Entity, &Room)>();
            let mut target = None;
            for (entity, room) in query.iter_mut(world) {
                if room.name == room_name {
                    target = Some(entity);
                }
            }
            match target {
                Some(room) => world.entity_mut(room).insert(ActiveRoom {}),
                None => panic!(
                    "Attempted to set the active room to `{}`, which does not exist",
                    room_name
                ),
            };
        });
    }
}

/// Modify Bevy's EntityCommands struct
use crate::events::{Event, IntoEventHandler};
use bevy::prelude::Component;

/// Adds methods to Bevy's [EntityCommands] struct
pub trait AdventureEntityCommands {
    /// Bind an event to an entity. This is effectively a shortcut for `.insert(<EventType>::new(<EventHandler>.into_event()))`.
    ///
    /// Example:
    /// ```
    /// // Makes a new entity named 'door' that will change rooms when interacted with
    /// let doorway = commands
    ///     // Make a new entity & give it a `Name` component
    ///     .spawn(Name("door".to_owned()))
    ///     // Bind the OnInteract event to a closure that changes the active room
    ///     // The first generic is the event type, and the second is Params
    ///     // Don't worry about the second generic too much, it's just Bevy magic.
    ///     // Just put a `_` to make Rust autodetect it.
    ///     .bind_event::<OnInteract, _>(|mut commands: Commands|{
    ///         // Change the active room to the second room
    ///         commands.set_room("Room 2");
    ///     })
    ///     // Return the Entity to store in the doorway variable
    ///     .id();
    /// ```
    fn bind_event<EventType: Event + Component, Params>(
        &mut self,
        handler: impl IntoEventHandler<Params>,
    ) -> &mut Self;
}
impl AdventureEntityCommands for EntityCommands<'_, '_, '_> {
    fn bind_event<EventType: Event + Component, Params>(
        &mut self,
        handler: impl IntoEventHandler<Params>,
    ) -> &mut Self {
        self.insert(EventType::new(handler.into_event()))
    }
}
