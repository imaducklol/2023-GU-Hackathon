use crate::room_class_stuff::Room;
use crate::room_class_stuff::Connection;
use crate::item_class::Item;

pub struct World {
    pub EVIL_BAD_ERROR_ROOM : Room,
    pub BULLDOG_ALLEY_CENTRAL : Room,
    pub BULLDOG_ALLEY_EAST : Room,
    pub COLLEGE_HALL : Room,
    pub DESMET : Room,
    pub FOLEY_LAWN : Room,
    pub HEMMINGSTON : Room,
    pub HERAK_QUAD : Room,
}

impl World {
    pub fn create_world(&mut self) {
        self.EVIL_BAD_ERROR_ROOM = Default::default();
        self.EVIL_BAD_ERROR_ROOM.description = "A very bad an evil room.".to_string();
        self.EVIL_BAD_ERROR_ROOM.address = "NULL".to_string();
        //connections

        self.BULLDOG_ALLEY_CENTRAL = Default::default();

        self.BULLDOG_ALLEY_EAST = Default::default();
        self.BULLDOG_ALLEY_EAST.description = "Blah blah blah.".to_string();
        self.BULLDOG_ALLEY_EAST.address = "BULLDOG_ALLEY_EAST".to_string();
        self.BULLDOG_ALLEY_EAST.add_connection("BULLDOG_ALLEY_CENTRAL".to_string(), "WEST".to_string());

        self.BULLDOG_ALLEY_CENTRAL = Default::default();
        self.BULLDOG_ALLEY_CENTRAL.description = "Blah blah blah.".to_string();
        self.BULLDOG_ALLEY_CENTRAL.address = "BULLDOG_ALLEY_CENTRAL".to_string();   
        self.BULLDOG_ALLEY_CENTRAL.add_connection("BULLDOG_ALLEY_EAST".to_string(), "EAST".to_string());
        




    }

    pub fn change_room(&self, destination : String) -> &Room {
        match destination.as_str() {
            "BULLDOG_ALLEY_CENTRAL" => {
                return &self.BULLDOG_ALLEY_CENTRAL;
            }
            "BULLDOG_ALLEY_EAST" => {
                return &self.BULLDOG_ALLEY_EAST;
            }
            _ => {
                return &self.EVIL_BAD_ERROR_ROOM;
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        World{EVIL_BAD_ERROR_ROOM : Default::default(),
            BULLDOG_ALLEY_CENTRAL : Default::default(),
            BULLDOG_ALLEY_EAST : Default::default(),
            COLLEGE_HALL : Default::default(),
            DESMET : Default::default(),
            FOLEY_LAWN : Default::default(),
            HEMMINGSTON : Default::default(),
            HERAK_QUAD : Default::default(),}
    }
}    