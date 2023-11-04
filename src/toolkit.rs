pub fn roll_die(sides: &i32, modifier: &i32) -> i32 {
    // Import needed random stuff
    use rand::prelude::*;

    // Create our random generator
    let mut rng: ThreadRng = thread_rng();

    // Get a random number between 1 and sides inclusive
    let roll: i32 = rng.gen_range(1..=*sides);

    let final_value: i32 = roll + modifier;

    final_value
}

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}