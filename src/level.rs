use crate::EventHandler;

/*
level
    rooms
    boss
rooms
    monsters/enemies
    items/interactables
    events
*/

// Something in the room the player can interact with
pub struct Interactable {

}

// A room in the game
pub struct Room {
    on_enter_handler: Option<EventHandler>,
    description: String
}
impl Room {
    pub fn new() -> Self {
        Self {
            on_enter_handler: None,
            description: "You enter a room.".into()
        }
    }
    pub fn on_enter(mut self, handler: EventHandler) -> Self {
        self.on_enter_handler = Some(handler);

        self
    }
    pub fn set_description(mut self, new_description: String) -> Self {
        self.description = new_description;

        self
    }
}

// A game level (which is really a glorified vec of Rooms)
pub struct Level {
    rooms: Vec<Room>
}
impl Level {
    pub fn new() -> Self {
        Self {
            rooms: Vec::<Room>::new()
        }
    }
    pub fn add_room(mut self, room: Room) -> Self {
        self.rooms.push(room);

        self
    }
    pub fn add_rooms(mut self, rooms: Vec<Room>) -> Self {
        self.rooms.extend(rooms);

        self
    }
}
