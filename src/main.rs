// Use the "io" from the standard library
use std::io;

// A new fancy variable
fn read_input(message: &str) -> String {
    // Prints a message
    println!("{}", message);
    // Declare a new variable
    let mut input = String::new();
    // Read the line
    io::stdin()
        .read_line(&mut input)
        // It it fails, error out
        .expect("Failed to read line");
    // return the input variable which will be set on "io::stdin().read_line(&mut input)"
    input.trim().to_string()
}

// A function for parsing strings to a 64-bit integer
fn parse_f64(string: &str) -> f64 {
    string.parse().expect("Invalid number")
}

fn is_valid_operator(operator: &str) -> bool {
    ["+", "-", "*", "/"].contains(&operator)
}

fn main() {
    // Declare these variables as strings.
    // for both the first and second things, it'll be converted to a 64-bit integer later
    let first_thing = parse_f64(&read_input("Enter the first number to operate on:"));
    let operator = read_input("Enter your operator");

    if !is_valid_operator(operator.trim()) {
        println!("The string is not a valid operator.");
        return;
    }

    let second_thing = parse_f64(&read_input("Enter the second number to operate on:"));

    // If it matches a:
    let result = match operator.as_str() {
        // "+" add
        "+" => first_thing + second_thing,
        // "-" substract
        "-" => first_thing - second_thing,
        // "*" multiply
        "*" => first_thing * second_thing,
        // "/" divide
        "/" => first_thing / second_thing,
        // If it doesn't match any of these, prints an error
        _ => {
            println!("The string is not a valid operator.");
            return;
        }
    };

    println!("Result: {}", result);
}
