use std::default;

use crate::{input::take_input, world_maker::World};

mod player_class_stuff;
mod room_class_stuff;
mod extra_classes;
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

    command();

}

fn command() {
    //HELP, GO, INVESTIGATE
    let mut input_success = false; // Keep track of if we have successfully handled input.

    while input_success == false {
        println!("What would you like to do?");
        let input_command = take_input(); // Get input
        let splitted = input_command.split_once(" "); // Split into two parts

        // Check for HELP command
        if "HELP" == input_command {
            println!("Here are available commands: HELP, GO, INVESTIGATE");
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
                        println!("{}", left);
                        input_success = true;
                        continue;
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
