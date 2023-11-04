use crate::Room;
use crate::Connection;
use crate::Address;
use crate::Item;

struct World {
    BULLDOG_ALLEY_CENTRAL : Room,
    BULLDOG_ALLEY_EAST : Room,
    COLLEGE_HALL : Room,
    DESMET : Room,
    FOLEY_LAWN : Room,
    HEMMINGSTON : Room,
    HERAK_QUAD : Room,
}

impl World {
    fn create_world(&mut self) {
        BULLDOG_ALLEY_CENTRAL = Default::default();
        
        BULLDOG_ALLEY_CENTRAL.description = "";

        BULLDOG_ALLEY_CENTRAL.objects 
    }
}
