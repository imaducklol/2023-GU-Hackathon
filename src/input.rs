use std::f32::consts::E;

pub fn take_input () -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("ERROR: Could not properly read input");
    input = input.trim().parse().unwrap();
    input.to_uppercase()
}