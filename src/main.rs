use std::default;

use crate::{input::take_input, world_maker::World, room_class_stuff::Room, toolkit::clear_screen};
use crate::player_class_stuff::Player;

mod player_class_stuff;
mod room_class_stuff;
mod extra_classes;
mod input;
mod enemy_interactions;
mod toolkit;
mod world_maker;

fn main() {
    let mut world: World = Default::default();
    world.create_world();
    let mut player = Player {
        health: 10,
        intelligence: 3,
        strength: 3,
        inventory: vec![],
    };

    let mut current_room = &world.BULLDOG_ALLEY_CENTRAL;

    intro();

    loop {
        let mut next_address: String = "".to_string();
        command(current_room, &world, &mut next_address, &mut player);
        if next_address != "".to_string() {
            current_room = world.change_room(next_address.to_string());
            println!("{}", current_room.description);
        }
    }
}

fn intro() {
    println!("");
}

fn command(current_room: &Room, world: &World, destination: &mut String, player: &mut Player) {
    //HELP, GO, INVESTIGATE
    let mut input_success = false; // Keep track of if we have successfully handled input.

    while input_success == false {
        println!("What would you like to do?");
        let input_command = take_input(); // Get input
        let splitted = input_command.split_once(" "); // Split into two parts

        println!("\n ");
        clear_screen();


        // Check for HELP command
        if "HELP" == input_command {
            println!("Here are available commands: HELP, GO, INVESTIGATE, LOOK AROUND, GRAB, INVENTORY, USE, QUIT");
            input_success = true;
            continue;
        }

        // Check for LOOK AROUND command
        if "LOOK AROUND" == input_command {
            println!("{}", current_room.description);
            input_success = true;
            continue;
        }

        // Check for INVENTORY command
        if "INVENTORY" == input_command {
            player.print_inventory();
            input_success = true;
            continue;
        }

        // Check for QUIT command
        if "QUIT" == input_command {
            std::process::exit(0);
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
                            "CROSBY" => {
                                if player.get_item("ID CARD".to_string()).code_name != "NullItem".to_string() {
                                    println!("You use the Id Card to get into Crosby.");
                                    *destination = attempt_destination.clone();
                                    input_success = true;
                                    continue;
                                } else {
                                    println!("It seems that Crosby requires an Id Card to get in.");
                                }
                            }
                            _ => {
                                *destination = attempt_destination.clone();
                                input_success = true;
                                continue;
                            }
                        }
                    }
                    "INVESTIGATE" => {
                        //println!("{}", right);
                        let room_object = current_room.get_object(right.to_string());
                        let inv_item = player.get_item(right.to_string());
                        if inv_item.code_name != "NullItem".to_string() {
                            println!("{}", inv_item.description);
                            input_success = true;
                            continue;
                        } else if room_object.name != "NullObject".to_string() {
                            println!("{}", room_object.description);
                            input_success = true;
                            continue;
                        } else {
                            println!("I do not know that.");
                            input_success = true;
                            continue;
                        }
                    }

                    "GRAB" => {
                        let room_item = current_room.get_item(right.to_string());

                        if room_item.code_name != "NullItem".to_string() {
                            let player_item = player.get_item(right.to_string());
                            if player_item.code_name != "NullItem" {
                                println!("You already have that item.");
                                input_success = true;
                                continue;
                            } else {
                                println!("You grabbed {}.", room_item.display_name);
                                player.inventory.push(room_item);
                            }
                        } else {
                            println!("That's not here to grab.");
                            input_success = true;
                            continue;
                        }
                    }

                    "USE" => {
                        let room_object = current_room.get_object(right.to_string());
                        let player_item = player.get_item(right.to_string());
                        if player_item.code_name != "NullItem".to_string() {
                            (*world).use_thing(current_room.address.clone(), player_item.code_name);
                            input_success = true;
                            continue;
                        } else if room_object.name != "NullObject".to_string() {
                            (*world).use_thing(current_room.address.clone(), room_object.name);
                            input_success = true;
                            continue;
                        } else {
                            println!("I do not know that.");
                            input_success = true;
                            continue;
                        }
                    }

                    _ => {
                        println!("Try again, I don't know {}.", left);
                    }
                }
            }
        }
    }
}
