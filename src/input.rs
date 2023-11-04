pub fn take_input() -> String {
    // String to input to
    let mut input: String = String::new();

    // Actually get input
    std::io::stdin()
        .read_line(&mut input)
        .expect("ERROR: Could not properly read input");

    // Cleanse input of returns and other BS
    input = input.trim().parse().unwrap();

    // Return input in all caps for standardization
    input.to_uppercase()
}