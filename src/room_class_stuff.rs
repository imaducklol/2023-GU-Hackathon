use std::default;
use crate::extra_classes::{Item, RoomObject};

pub struct Connection {
    pub destination: String,
    pub name: String,
}

// Room that houses locations.
pub struct Room {
    pub connections: Vec<Connection>,
    pub description: String,
    pub address: String,
    pub items: Vec<Item>,
    pub objects: Vec<RoomObject>,

    //TODO - Add other things
}

impl Room {
    // Adds a connection to the room.
    pub fn add_connection(&mut self, destination: String, name: String) {
        let new_connection = Connection { destination: destination, name: name };
        self.connections.push(new_connection);
    }

    pub fn add_object(&mut self, object_name: &String, object_description: &String) {
        let new_object = RoomObject { name: (*object_name).clone(), description: (*object_description).clone() };
        self.objects.push(new_object)
    }

    pub fn add_item(&mut self, item_name: &String, item_description: &String, item_tags: &Vec<String>) {
        let new_item = Item { name: (*item_name).clone(), description: (*item_description).clone(), tags: (*item_tags).clone() };
        self.items.push(new_item)
    }

    pub fn add_objects(&mut self, object_names: Vec<String>, object_descriptions: Vec<String>) {
        if object_names.len() != object_descriptions.len() {
            panic!("While adding objects, number of names and number of descriptions do not match!");
        }
        for i in 0..object_names.len() {
            self.add_object(&object_names[i], &object_descriptions[i])
        }
    }

    pub fn add_items(&mut self, item_names: Vec<String>, item_descriptions: Vec<String>, item_tags_s: Vec<Vec<String>>) {
        if item_names.len() != item_descriptions.len() {
            panic!("While adding items, number of names and number of descriptions do not match!");
        }
        for i in 0..item_names.len() {
            self.add_item(&item_names[i], &item_descriptions[i], &item_tags_s[i])
        }
    }

    // Adds multiple connections from a list to the room.
    /*pub fn add_connections(&mut self, new_connection_vector : Vec<Connection>) {
        for connection in new_connection_vector {
            self.add_connection(connection);
        }
    }*/

    // Gets a room address given a string.
    pub fn get_room_destination(&self, name : String) -> String {
        let mut room = "Nullroom".to_string();

        for connection in &(*self).connections {
            if connection.name == name {
                room = (*connection).destination.clone();
            }
        }

        return room;
    }

    // Gets an Item given a name
    pub fn get_item(self, name: String) -> Item {
        let mut found_item: Item = Item { name: String::from("NullItem"), description: String::from("NullItem"), tags: Vec::new() };

        for item in self.items {
            if item.name == name {
                found_item = item;
            }
        }

        return found_item;
    }

    // Gets an Object given a name
    pub fn get_object(self, name: String) -> RoomObject {
        let mut found_object: RoomObject = RoomObject { name: String::from("NullObject"), description: String::from("NullObject") };

        for object in self.objects {
            if object.name == name {
                found_object = object;
            }
        }

        return found_object;
    }
}

// Room default constructor.
impl Default for Room {
    fn default() -> Self {
        Self { connections: Vec::new(), description: String::from("Empty"), items: Vec::new(), objects: Vec::new(), address: String::from("Nullroom") }
    }
}