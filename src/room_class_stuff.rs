use std::default;
use crate::item_class::Item;

pub struct Connection {
    pub destination : String,
    pub name : String,
}

// Room that houses locations.
pub struct Room {
    pub connections : Vec<Connection>,
    pub description : String,
    pub address : String,
    pub objects : Vec<Item>,

    //TODO - Add other things
}

impl Room {
    // Adds a connection to the room.
    pub fn add_connection(&mut self, destination : String, name : String) {
        let new_connection = Connection { destination: destination, name: name };
        self.connections.push(new_connection);
    }

    // Adds multiple connections from a list to the room.
    /*pub fn add_connections(&mut self, new_connection_vector : Vec<Connection>) {
        for connection in new_connection_vector {
            self.add_connection(connection);
        }
    }*/

    // Gets a room address given a string.
    pub fn get_room_destination(self, name : String) -> String {
        let mut room = "Nullroom".to_string();

        for connection in self.connections {
            if connection.name == name {
                room = connection.destination;
            }
        }

        return room;
    }

    // Gets an Item given a name
    pub fn get_item(self, name : String) -> Item {
        let mut found_item : Item = Item{name : String::from("NullItem"), tags : Vec::new()};

        for item in self.objects {
            if item.name == name {
                found_item = item;
            }
        }

        return found_item;
    }

}

// Room default constructor.
impl Default for Room {
    fn default() -> Self {
        Self {connections: Vec::new(), description: String::from("Empty"), objects : Vec::new(), address: String::from("Nullroom")}
    }
}