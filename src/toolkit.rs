
pub fn roll_die(sides: i32, modifier: i32) -> i32 {
    use rand::Rng;
    let mut rng: ThreadRng = thread_rng();
    let roll: i32 = rng.gen_range(1..=sides);
    let final_value: i32 = roll + modifier;
    final_value
}