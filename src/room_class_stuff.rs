use std::default;
use crate::item_class::Item;

// Stores the address of a room, or Nil if empty.
pub enum Address {
    Address(Box<Room>), // To use this, you need to use Box::new(insert_room_here)
    Nil
}

// Connection, stores an Address as well as a name of the connection.
pub struct Connection {
    pub name : String,
    pub address : Address
}

// Default constructor
impl Default for Connection {
    fn default() -> Self {
        Connection { name: String::from("Empty"), address: Address::Nil }
    }
}

// Room that houses locations.
pub struct Room {
    connections : Vec<Connection>,
    description : String,
    objects : Vec<Item>,

    //TODO - Add other things
}

impl Room {
    // Adds a connection to the room.
    pub fn add_connection(&mut self, new_connection : Connection) {
        self.connections.push(new_connection);
    }

    // Adds multiple connections from a list to the room.
    pub fn add_connections(&mut self, new_connection_vector : Vec<Connection>) {
        for connection in new_connection_vector {
            self.add_connection(connection);
        }
    }

    // Prints all connections to the room.
    pub fn print_connections(self) {
        for connection in self.connections {
            match connection.address {
                Address::Nil => {
                    println!("Empty connection!");
                }

                Address::Address(connected_room) => {
                    println!("{}", connected_room.description);
                }
            }
        }
    }

    // Gets a room address given a string.
    pub fn get_room_address(self, name : String) -> Address {
        let mut room = Address::Nil;

        for connection in self.connections {
            if connection.name == name {
                room = connection.address;
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
        Self {connections: Vec::new(), description: String::from("Empty"), objects : Vec::new() }
    }
}