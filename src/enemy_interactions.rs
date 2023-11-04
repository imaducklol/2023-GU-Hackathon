use crate::input::take_input;
use crate::player_class_stuff::Player;
use crate::toolkit;

enum EncounterState {
    DefaultValue,
    EvadeSuccess,
    EvadeFailure,
    AttackSuccess,
    AttackFailure,
}

pub fn encounter_enemy(player: Player, evade_dc: i32, attack_dc: i32) -> EncounterState {
    println!("You've encountered an enemy!");
    println!("Do you want to evade (1), or attack (2)?!");

    let mut input: String;

    loop {
        input = take_input();
        if input != "e" || input != "a" {
            println!("That's not a valid input, please type (a) or (e)");
            continue;
        } else {
            break;
        }
    }

    let mut encounter_result: EncounterState = EncounterState::DefaultValue;

    match input.as_str() {
        "e" => {
            println!("You attempt to evade Sodexo's evil Food Delivery Machine!");
            println!("Roll to evade! (Enter)");
            let _ = take_input();
            let roll: i32 = toolkit::roll_die(20, player.intelligence);
            if roll > evade_dc {
                println!("You successfully evaded!");
                encounter_result = EncounterState::EvadeSuccess;
            } else {
                println!("You failed to evade and were found by the evil Sodexo Robot!");
                encounter_result = EncounterState::EvadeFailure;
            }
        }
        "a" => {
            println!("You attempt to destroy Sodexo's evil Food Delivery Machine!");
            println!("Roll to attack! (Enter)");
            let _ = take_input();
            let roll: i32 = toolkit::roll_die(20, player.strength);
            if roll > attack_dc {
                println!("You successfully destroyed the bot!");
                encounter_result = EncounterState::AttackSuccess;
            } else {
                println!("You failed destroy the evil Sodexo Robot!");
                encounter_result = EncounterState::AttackFailure;
            }
        }
        _ => { panic!("Something has gone terribly wrong in the encounter match function! Debug your code Orion"); }
    };

    if encounter_result == EncounterState::DefaultValue {
        panic!("Encounter state was not set, something has gone wrong");
    }

    encounter_result
}