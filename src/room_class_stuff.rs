use std::default;

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
    description : String
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
}

// Room default constructor.
impl Default for Room {
    fn default() -> Self {
        Self {connections: Vec::new(), description: String::from("Empty") }
    }
}