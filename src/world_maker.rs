use crate::room_class_stuff::Room;
use crate::room_class_stuff::Connection;
use crate::room_class_stuff::Address;
use crate::item_class::Item;
use std::rc::Rc;

pub struct World {
    pub BULLDOG_ALLEY_CENTRAL : Address,
    pub BULLDOG_ALLEY_EAST : Address,
    pub COLLEGE_HALL : Address,
    pub DESMET : Address,
    pub FOLEY_LAWN : Address,
    pub HEMMINGSTON : Address,
    pub HERAK_QUAD : Address,
}

impl World {
    // Makes a address with a room in it
    pub fn room_address() -> Address {
        Address::room(Rc::new(Default::default()))
    }

    pub fn create_world(&mut self) {
        self.BULLDOG_ALLEY_CENTRAL = Self::room_address();
        self.BULLDOG_ALLEY_EAST = Self::room_address();
        self.DESMET = Self::room_address();
        self.HEMMINGSTON = Self::room_address();
        self.FOLEY_LAWN = Self::room_address();


        self.BULLDOG_ALLEY_CENTRAL.get_room().add_connection_room(Rc::new(self.BULLDOG_ALLEY_EAST), "Bulldog Alley East".to_string());
        
        
        self.BULLDOG_ALLEY_CENTRAL.get_room().description = "Bulldog Alley Central. The common path that all GU students walk. It is barren now. Bulldog Alley East lies ahead.".to_string();
        self.BULLDOG_ALLEY_EAST.get_room().description = "Bulldog Alley East. The common path that all GU students walk. It is barren and empty. A liminal space. Bulldog Alley Central lies ahead.".to_string();
    }
}
