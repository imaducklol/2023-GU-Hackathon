use std::default;

use crate::{input::take_input, world_maker::World};
mod player_class_stuff;
mod room_class_stuff;
mod item_class;
mod input;
mod enemy_interactions;
mod toolkit;
mod world_maker;

fn main() {
    println!("Hello, world!");
    let mut world : World = Default::default();
    world.create_world();

    let mut current_room = &world.BULLDOG_ALLEY_CENTRAL;

    println!("{}", current_room.address);
    current_room = world.change_room("BULLDOG_ALLEY_EAST".to_string());
    println!("{}", current_room.address);

}
