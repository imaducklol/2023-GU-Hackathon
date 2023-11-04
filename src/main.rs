use std::default;

use crate::{input::take_input, world_maker::World, room_class_stuff::Room, toolkit::clear_screen};

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

    loop {
        let mut next_address : String = "".to_string();
        command(current_room, &world, &mut next_address);
        if next_address != "".to_string() {
            current_room = world.change_room(next_address.to_string());
            println!("{}", current_room.description);
        }
    }

}

fn command(current_room : &Room, world : &World, destination : &mut String) {
    //HELP, GO, INVESTIGATE
    let mut input_success = false; // Keep track of if we have successfully handled input.

    while input_success == false {
        println!("What would you like to do?");
        let input_command = take_input(); // Get input
        let splitted = input_command.split_once(" "); // Split into two parts

        // Check for HELP command
        if "HELP" == input_command {
            println!("Here are available commands: HELP, GO, INVESTIGATE, LOOK AROUND");
            input_success = true;
            continue;
        }

        // Check for LOOK AROUND command
        if "LOOK AROUND" == input_command {
            println!("{}", current_room.description);
            input_success = true;
            continue;
          }

        // Check to see if it was split
        match splitted {
            None => {
                println!("I'm sorry, I don't understand {}", input_command);
            }
            _ => {
                let (left, right) = splitted.unwrap();
                // Check the first part
                match left {
                    "GO" => {
                        let attempted_move_name = right.to_string();
                        let attempt_destination = (*current_room).get_room_destination(attempted_move_name);

                        match attempt_destination.as_str() {
                            "Nullroom" => {
                                println!("I don't know how to get there.");
                            }
                            _ => {
                                *destination = attempt_destination.clone();
                                input_success = true;
                                continue;
                            }
                        }
                    }
                    "INVESTIGATE" => {
                        println!("{}", left);
                        input_success = true;
                        continue;
                    }
                    _ => {
                        println!("Try again, I don't know {}.", left);
                    }
                }
        
            }
        }
    }


}
