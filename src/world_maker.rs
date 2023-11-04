use crate::room_class_stuff::Room;
use crate::room_class_stuff::Connection;
use crate::item_class::Item;

pub struct World {
    pub EVIL_BAD_ERROR_ROOM : Room,
    pub BULLDOG_ALLEY_CENTRAL : Room,
    pub BULLDOG_ALLEY_EAST : Room,
    pub COLLEGE_HALL : Room,
    pub CROSBY : Room,
    pub DESMET : Room,
    pub FOLEY_LAWN : Room,
    pub FOLEY_LIBRARY : Room,
    pub HEMMINGSON : Room,
    pub HERAK_QUAD : Room,
    pub HUGHES: Room,
    pub PATHWAYS : Room,
    pub ROSAUER : Room,
    pub WELCH : Room,
}

impl World {
    pub fn create_world(&mut self) {
        self.EVIL_BAD_ERROR_ROOM = Default::default();
        self.EVIL_BAD_ERROR_ROOM.description = "A very bad an evil room.".to_string();
        self.EVIL_BAD_ERROR_ROOM.address = "NULL".to_string();

        //connections

        /*
        self.NAME = Default::default();
        self.NAME.description = "Blah blah blah.".to_string();
        self.NAME.address = "NAME".to_string();
        self.NAME.add_connection("DESTINATION".to_string(), "PLACEHOLDER".to_string());
         */

        self.BULLDOG_ALLEY_CENTRAL = Default::default();
        self.BULLDOG_ALLEY_CENTRAL.description = "Blah blah blah.".to_string();
        self.BULLDOG_ALLEY_CENTRAL.address = "BULLDOG_ALLEY_CENTRAL".to_string();
        self.BULLDOG_ALLEY_CENTRAL.add_connection("COLLEGE_HALL".to_string(), "PLACEHOLDER".to_string());
        self.BULLDOG_ALLEY_CENTRAL.add_connection("CROSBY".to_string(), "PLACEHOLDER".to_string());
        self.BULLDOG_ALLEY_CENTRAL.add_connection("DESMET".to_string(), "PLACEHOLDER".to_string());
        self.BULLDOG_ALLEY_CENTRAL.add_connection("HERAK_QUAD".to_string(), "PLACEHOLDER".to_string());
        self.BULLDOG_ALLEY_CENTRAL.add_connection("BULLDOG_ALLEY_EAST".to_string(), "PLACEHOLDER".to_string());

        self.BULLDOG_ALLEY_EAST = Default::default();
        self.BULLDOG_ALLEY_EAST.description = "Blah blah blah.".to_string();
        self.BULLDOG_ALLEY_EAST.address = "BULLDOG_ALLEY_EAST".to_string();
        self.BULLDOG_ALLEY_EAST.add_connection("BULLDOG_ALLEY_CENTRAL".to_string(), "PLACEHOLDER".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("CROSBY".to_string(), "PLACEHOLDER".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("DESMET".to_string(), "PLACEHOLDER".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("FOLEY_LAWN".to_string(), "PLACEHOLDER".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("HEMMINGSON".to_string(), "PLACEHOLDER".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("ROSAUER".to_string(), "PLACEHOLDER".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("WELCH".to_string(), "PLACEHOLDER".to_string());

        self.COLLEGE_HALL = Default::default();
        self.COLLEGE_HALL.description = "Blah blah blah.".to_string();
        self.COLLEGE_HALL.address = "COLLEGE_HALL".to_string();
        self.COLLEGE_HALL.add_connection("BULLDOG_ALLEY_CENTRAL".to_string(), "PLACEHOLDER".to_string());

        self.CROSBY = Default::default();
        self.CROSBY.description = "Blah blah blah.".to_string();
        self.CROSBY.address = "CROSBY".to_string();
        self.CROSBY.add_connection("BULLDOG_ALLEY_CENTRAL".to_string(), "PLACEHOLDER".to_string());
        self.CROSBY.add_connection("BULLDOG_ALLEY_EAST".to_string(), "PLACEHOLDER".to_string());
        self.CROSBY.add_connection("FOLEY_LAWN".to_string(), "PLACEHOLDER".to_string());
        self.CROSBY.add_connection("HERAK_QUAD".to_string(), "PLACEHOLDER".to_string());

        self.DESMET = Default::default();
        self.DESMET.description = "Blah blah blah.".to_string();
        self.DESMET.address = "DESMET".to_string();
        self.DESMET.add_connection("BULLDOG_ALLEY_EAST".to_string(), "PLACEHOLDER".to_string());
        self.DESMET.add_connection("BULLDOG_ALLEY_CENTRAL".to_string(), "PLACEHOLDER".to_string());
        self.DESMET.add_connection("PATHWAYS".to_string(), "PLACEHOLDER".to_string());

        self.HEMMINGSON = Default::default();
        self.HEMMINGSON.description = "Blah blah blah.".to_string();
        self.HEMMINGSON.address = "HEMMINGSON".to_string();
        self.HEMMINGSON.add_connection("BULLDOG_ALLEY_EAST".to_string(), "PLACEHOLDER".to_string());
        self.HEMMINGSON.add_connection("FOLEY_LAWN".to_string(), "PLACEHOLDER".to_string());

        self.HERAK_QUAD = Default::default();
        self.HERAK_QUAD.description = "Blah blah blah.".to_string();
        self.HERAK_QUAD.address = "HERAK_QUAD".to_string();
        self.HERAK_QUAD.add_connection("CROSBY".to_string(), "PLACEHOLDER".to_string());
        self.HERAK_QUAD.add_connection("HUGHES".to_string(), "PLACEHOLDER".to_string());
        self.HERAK_QUAD.add_connection("BULLDOG_ALLEY_CENTRAL".to_string(), "PLACEHOLDER".to_string());

        self.HUGHES = Default::default();
        self.HUGHES.description = "Blah blah blah.".to_string();
        self.HUGHES.address = "HUGHES".to_string();
        self.HUGHES.add_connection("HERAK_QUAD".to_string(), "PLACEHOLDER".to_string());

        self.PATHWAYS = Default::default();
        self.PATHWAYS.description = "Blah blah blah.".to_string();
        self.PATHWAYS.address = "PATHWAYS".to_string();
        self.PATHWAYS.add_connection("DESMET".to_string(), "PLACEHOLDER".to_string());
        self.PATHWAYS.add_connection("FOLEY_LAWN".to_string(), "PLACEHOLDER".to_string());
        self.PATHWAYS.add_connection("WELCH".to_string(), "PLACEHOLDER".to_string());

        self.ROSAUER = Default::default();
        self.ROSAUER.description = "Blah blah blah.".to_string();
        self.ROSAUER.address = "ROSAUER".to_string();
        self.ROSAUER.add_connection("BULLDOG_ALLEY_EAST".to_string(), "PLACEHOLDER".to_string());

        self.WELCH = Default::default();
        self.WELCH.description = "Blah blah blah.".to_string();
        self.WELCH.address = "WELCH".to_string();
        self.WELCH.add_connection("BULLDOG_ALLEY_EAST".to_string(), "PLACEHOLDER".to_string());
        self.WELCH.add_connection("PATHWAYS".to_string(), "PLACEHOLDER".to_string());





    }

    pub fn change_room(&self, destination : String) -> &Room {
        match destination.as_str() {
            "BULLDOG_ALLEY_CENTRAL" => {
                return &self.BULLDOG_ALLEY_CENTRAL;
            }
            "BULLDOG_ALLEY_EAST" => {
                return &self.BULLDOG_ALLEY_EAST;
            }
            "COLLEGE_HALL" => {
                return &self.COLLEGE_HALL;
            }
            "CROSBY" => {
                return &self.CROSBY;
            }
            "DESMET" => {
                return &self.DESMET;
            }
            "FOLEY_LAWN" => {
                return &self.FOLEY_LAWN;
            }
            "FOLEY_LIBRARY" => {
                return &self.FOLEY_LIBRARY;
            }
            "HEMMINGSON" => {
                return &self.HEMMINGSON;
            }
            "HERAK_QUAD" => {
                return &self.HERAK_QUAD;
            }
            "HUGHES" => {
                return &self.HUGHES;
            }
            "PATHWAYS" => {
                return &self.PATHWAYS;
            }
            "ROSAUER" => {
                return &self.ROSAUER;
            }
            "WELCH" => {
                return &self.WELCH;
            }
            _ => {
                return &self.EVIL_BAD_ERROR_ROOM;
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        // Default Constructor my beloved.
        World{
            EVIL_BAD_ERROR_ROOM : Default::default(),
            BULLDOG_ALLEY_CENTRAL : Default::default(),
            BULLDOG_ALLEY_EAST : Default::default(),
            COLLEGE_HALL : Default::default(),
            CROSBY : Default::default(),
            DESMET : Default::default(),
            FOLEY_LAWN : Default::default(),
            FOLEY_LIBRARY : Default::default(),
            HEMMINGSON : Default::default(),
            HERAK_QUAD : Default::default(),
            HUGHES : Default::default(),
            PATHWAYS : Default::default(),
            ROSAUER : Default::default(),
            WELCH : Default::default(),
        }
    }
}    