use std::{io, thread};
use std::io::Write;

/*pub fn roll_die(sides: &i32, modifier: &i32) -> i32 {
    // Import needed random stuff
    use rand::prelude::*;

    // Create our random generator
    let mut rng: ThreadRng = thread_rng();

    // Get a random number between 1 and sides inclusive
    let roll: i32 = rng.gen_range(1..=*sides);

    let final_value: i32 = roll + modifier;

    final_value
}*/

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn print_gap_clear() {
    print!("\n \n ");
    clear_screen();
}

pub fn sleep(seconds: f64) {
    use std::time;
    thread::sleep(time::Duration::from_micros((seconds * 1_000_000f64) as u64));
}

pub fn fancy_println(text: String, sleep_time_seconds: f64) {
    let char_vec: Vec<char> = text.chars().collect();
    for char in char_vec {
        print!("{}", char);
        let _ = io::stdout().flush();
        sleep(sleep_time_seconds);
    }
    println!();
}