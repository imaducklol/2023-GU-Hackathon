use crate::room_class_stuff::Room;
use crate::room_class_stuff::Connection;
use crate::extra_classes::Item;

pub struct World {
    pub EVIL_BAD_ERROR_ROOM: Room,
    pub BULLDOG_ALLEY_CENTRAL: Room,
    pub BULLDOG_ALLEY_EAST: Room,
    pub COLLEGE_HALL: Room,
    pub CROSBY: Room,
    pub DESMET: Room,
    pub FOLEY_LAWN: Room,
    pub FOLEY_LIBRARY: Room,
    pub HEMMINGSON: Room,
    pub HERAK_QUAD: Room,
    pub HUGHES: Room,
    pub PATHWAYS: Room,
    pub ROSAUER: Room,
    pub WELCH: Room,
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
        self.BULLDOG_ALLEY_CENTRAL.description = "You are in the center of Bulldog Alley, from here \
        you can see (College Hall), (Crosby), (Desmet), (Herak Quad), and further down Bulldog Alley to the (East). ".to_string();
        self.BULLDOG_ALLEY_CENTRAL.address = "BULLDOG_ALLEY_CENTRAL".to_string();
        self.BULLDOG_ALLEY_CENTRAL.add_connection("COLLEGE_HALL".to_string(), "COLLEGE HALL".to_string());
        self.BULLDOG_ALLEY_CENTRAL.add_connection("CROSBY".to_string(), "CROSBY".to_string());
        self.BULLDOG_ALLEY_CENTRAL.add_connection("DESMET".to_string(), "DESMET".to_string());
        self.BULLDOG_ALLEY_CENTRAL.add_connection("HERAK_QUAD".to_string(), "HERAK QUAD Quad".to_string());
        self.BULLDOG_ALLEY_CENTRAL.add_connection("BULLDOG_ALLEY_EAST".to_string(), "EAST".to_string());
        self.BULLDOG_ALLEY_CENTRAL.add_objects(vec!("TREE".to_string(), "SHRUB".to_string(), "BUSH".to_string()),
                                               vec!("There is a corpse in the tree".to_string(), "There is a (Red Keycard) in the shrub!".to_string(), "There are small pieces of broken robot in the bush".to_string()));
        self.BULLDOG_ALLEY_CENTRAL.add_item(&"RED KEYCARD".to_string(), &"this key does something".to_string(), &"Red Keycard".to_string());

        self.BULLDOG_ALLEY_EAST = Default::default();
        self.BULLDOG_ALLEY_EAST.description = "You are in the east part of Bulldog Alley, from here \
        you can see (Crosby), (Desmet), (Foley Lawn), (Hemmingson), (Rosauer), (Welch), further down Bulldog Alley to the (West), and the (Pathways) between Welch and Desmet.".to_string();
        self.BULLDOG_ALLEY_EAST.address = "BULLDOG_ALLEY_EAST".to_string();
        self.BULLDOG_ALLEY_EAST.add_connection("BULLDOG_ALLEY_CENTRAL".to_string(), "WEST".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("CROSBY".to_string(), "CROSBY".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("DESMET".to_string(), "DESMET".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("FOLEY_LAWN".to_string(), "FOLEY LAWN".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("HEMMINGSON".to_string(), "HEMMINGSON".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("ROSAUER".to_string(), "ROSAUER".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("WELCH".to_string(), "WELCH".to_string());
        self.BULLDOG_ALLEY_EAST.add_connection("PATHWAYS".to_string(), "PATHWAYS".to_string());

        self.COLLEGE_HALL = Default::default();
        self.COLLEGE_HALL.description = "You are inside of College Hall; you can see the door back out to (Bulldog Alley).".to_string();
        self.COLLEGE_HALL.address = "COLLEGE_HALL".to_string();
        self.COLLEGE_HALL.add_connection("BULLDOG_ALLEY_CENTRAL".to_string(), "BULLDOG ALLEY".to_string());

        self.CROSBY = Default::default();
        self.CROSBY.description = "You are inside of Crosby, you can see the door(s) back out to (Central) Bulldog Alley, (East) Bulldog Alley, (Foley Lawn), and (Herak Quad).".to_string();
        self.CROSBY.address = "CROSBY".to_string();
        self.CROSBY.add_connection("BULLDOG_ALLEY_CENTRAL".to_string(), "CENTRAL".to_string());
        self.CROSBY.add_connection("BULLDOG_ALLEY_EAST".to_string(), "EAST".to_string());
        self.CROSBY.add_connection("FOLEY_LAWN".to_string(), "FOLEY LAWN".to_string());
        self.CROSBY.add_connection("HERAK_QUAD".to_string(), "HERAK QUAD".to_string());

        self.DESMET = Default::default();
        self.DESMET.description = "You are inside of Desmet, you can see the door(s) back out to (Central) Bulldog Alley, (East) Bulldog Alley, and the (Pathways).".to_string();
        self.DESMET.address = "DESMET".to_string();
        self.DESMET.add_connection("BULLDOG_ALLEY_EAST".to_string(), "EAST".to_string());
        self.DESMET.add_connection("BULLDOG_ALLEY_CENTRAL".to_string(), "CENTRAL".to_string());
        self.DESMET.add_connection("PATHWAYS".to_string(), "PATHWAYS".to_string());

        self.FOLEY_LAWN = Default::default();
        self.FOLEY_LAWN.description = "Blah blah blah.".to_string();
        self.FOLEY_LAWN.address = "FOLEY_LAWN".to_string();
        self.FOLEY_LAWN.add_connection("DESTINATION".to_string(), "PLACEHOLDER".to_string());
        self.FOLEY_LAWN.add_connection("DESTINATION".to_string(), "PLACEHOLDER".to_string());
        self.FOLEY_LAWN.add_connection("DESTINATION".to_string(), "PLACEHOLDER".to_string());
        self.FOLEY_LAWN.add_connection("DESTINATION".to_string(), "PLACEHOLDER".to_string());
        self.FOLEY_LAWN.add_connection("DESTINATION".to_string(), "PLACEHOLDER".to_string());

        self.FOLEY_LIBRARY = Default::default();
        self.FOLEY_LIBRARY.description = "Blah blah blah.".to_string();
        self.FOLEY_LIBRARY.address = "FOLEY_LIBRARY".to_string();
        self.FOLEY_LIBRARY.add_connection("DESTINATION".to_string(), "PLACEHOLDER".to_string());

        self.HEMMINGSON = Default::default();
        self.HEMMINGSON.description = "You are inside of Hemmingson, you can see the door(s) back out to (East) Bulldog Alley, and (Foley Lawn).".to_string();
        self.HEMMINGSON.address = "HEMMINGSON".to_string();
        self.HEMMINGSON.add_connection("BULLDOG_ALLEY_EAST".to_string(), "EAST".to_string());
        self.HEMMINGSON.add_connection("FOLEY_LAWN".to_string(), "FOLEY LAWN".to_string());

        self.HERAK_QUAD = Default::default();
        self.HERAK_QUAD.description = "You are on the Herak Quad; from here you can see (Crosby), (Hughes), and (Central) Bulldog Alley.".to_string();
        self.HERAK_QUAD.address = "HERAK_QUAD".to_string();
        self.HERAK_QUAD.add_connection("CROSBY".to_string(), "CROSBY".to_string());
        self.HERAK_QUAD.add_connection("HUGHES".to_string(), "HUGHES".to_string());
        self.HERAK_QUAD.add_connection("BULLDOG_ALLEY_CENTRAL".to_string(), "CENTRAL".to_string());

        self.HUGHES = Default::default();
        self.HUGHES.description = "You are inside of HUGHES, you can see the door back out to (Herak Quad)".to_string();
        self.HUGHES.address = "HUGHES".to_string();
        self.HUGHES.add_connection("HERAK_QUAD".to_string(), "HERAK QUAD".to_string());

        self.PATHWAYS = Default::default();
        self.PATHWAYS.description = "You are in the pathways between (Desmet) and (Welch). From here you can see those buildings in addition to (Foley Lawn) and the (East) side of Bulldog Alley.".to_string();
        self.PATHWAYS.address = "PATHWAYS".to_string();
        self.PATHWAYS.add_connection("DESMET".to_string(), "DESMET".to_string());
        self.PATHWAYS.add_connection("FOLEY_LAWN".to_string(), "FOLEY LAWN".to_string());
        self.PATHWAYS.add_connection("WELCH".to_string(), "WELCH".to_string());
        self.PATHWAYS.add_connection("BULLDOG_ALLEY_EAST".to_string(), "EAST".to_string());

        self.ROSAUER = Default::default();
        self.ROSAUER.description = "You are inside of ROSAUER, you can see the door back out to the (East) side of Bulldog Alley.".to_string();
        self.ROSAUER.address = "ROSAUER".to_string();
        self.ROSAUER.add_connection("BULLDOG_ALLEY_EAST".to_string(), "EAST".to_string());

        self.WELCH = Default::default();
        self.WELCH.description = "You are inside of WELCH, you can see the doors back out to the (East) side of Bulldog Alley and to the (Pathways) between Welch and Desmet.".to_string();
        self.WELCH.address = "WELCH".to_string();
        self.WELCH.add_connection("BULLDOG_ALLEY_EAST".to_string(), "PLACEHOLDER".to_string());
        self.WELCH.add_connection("PATHWAYS".to_string(), "PLACEHOLDER".to_string());
    }

    pub fn change_room(&self, destination: String) -> &Room {
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

    //always input tag, not name
    pub fn use_thing(&mut self, room: String, tag: String) {
        let tag = tag.as_str();
        match room.as_str() {
            "BULLDOG_ALLEY_CENTRAL" => {
                match tag {
                    _ => {}
                }
            }
            "BULLDOG_ALLEY_EAST" => {
                match tag {
                    _ => {}
                }
            }
            "COLLEGE_HALL" => {
                match tag {
                    _ => {}
                }
            }
            "CROSBY" => {
                match tag {
                    _ => {}
                }
            }
            "DESMET" => {
                match tag {
                    _ => {}
                }
            }
            "FOLEY_LAWN" => {
                match tag {
                    _ => {}
                }
            }
            "FOLEY_LIBRARY" => {
                match tag {
                    "LAPTOP_1" => {
                        println!("You shake the mouse to wake up the laptop. As it boots up, you scroll through an article.");
                        println!("BREAKING - SODEXO FOOD ROBOTS MALFUNCTION");
                        println!("Published December 20, 2023");
                        println!("Although many enjoy going home over the holidays, not everyone gets to do so. Some students remain on-campus, enjoying their holidays at the wonderful GU.");
                        println!("However, some never get the luxury of going home. Corion Ilstess was found dead this morning in Foley.");
                        println!("Early this morning, Corion ordered a sandwhich from Iggy's, delivered by the Sodexo food robots. However, as the robot approached, disaster struck.
                        The robot approached Corion, and exploded. ");
                    }
                    _ => {}
                }
            }
            "HEMMINGSON" => {
                match tag {
                    "HACKING_LAPTOP" => {
                        println!("You open the laptop and boot it up. As you finish entering the code, you hear a slow buzz. All around you are Sodexo food robots. They have heart eyes.");
                        println!();
                        println!("The end.");
                    }
                    _ => {}
                }
            }
            "HERAK_QUAD" => {
                match tag {
                    _ => {}
                }
            }
            "HUGHES" => {
                match tag {
                    _ => {}
                }
            }
            "PATHWAYS" => {
                match tag {
                    "SQUIRREL_FOOD" => {
                        println!("A squirrel approaches you and takes the food.");
                        println!("You are filled with determination.");
                    }
                    _ => {}
                }
            }
            "ROSAUER" => {
                match tag {
                    _ => {}
                }
            }
            "WELCH" => {
                match tag {
                    _ => {}
                }
            }
            _ => {}
        }
    }
}


impl Default for World {
    fn default() -> Self {
        // Default Constructor my beloved.
        World {
            EVIL_BAD_ERROR_ROOM: Default::default(),
            BULLDOG_ALLEY_CENTRAL: Default::default(),
            BULLDOG_ALLEY_EAST: Default::default(),
            COLLEGE_HALL: Default::default(),
            CROSBY: Default::default(),
            DESMET: Default::default(),
            FOLEY_LAWN: Default::default(),
            FOLEY_LIBRARY: Default::default(),
            HEMMINGSON: Default::default(),
            HERAK_QUAD: Default::default(),
            HUGHES: Default::default(),
            PATHWAYS: Default::default(),
            ROSAUER: Default::default(),
            WELCH: Default::default(),
        }
    }
}    