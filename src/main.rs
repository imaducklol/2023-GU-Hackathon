use crate::input::take_input;
mod player_class_stuff;
mod room_class_stuff;
mod item_class;
mod input;
mod world_maker;

fn main() {
    let mut world : world_maker::World;
    world.create_world();

    let mut room : room_class_stuff::Room;
    room = world.BULLDOG_ALLEY_CENTRAL.get_room();

    room.print_connections();

    println!("Hello, world!");
}
