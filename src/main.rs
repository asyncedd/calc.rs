use lazy_static::lazy_static;
use std::collections::HashSet;
use std::io;

fn read_input(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn parse_f64(string: &str) -> f64 {
    string.parse().expect("Invalid number")
}

const VALID_OPERATORS: &[&str; 23] = &[
    "+", "-", "*", "/", "^", "sqrt", "sine", "cosine", "tangent", "abs", "floor", "ceiling", "tan",
    "asin", "acos", "ln", "log", "e ^", "sinh", "cosh", "tanh", "atan2", "atan",
];

fn join_valid_operators() -> String {
    VALID_OPERATORS.join(", ")
}

lazy_static! {
    static ref VALID_OPERATORS_SET: HashSet<&'static str> = {
        let set: HashSet<_> = VALID_OPERATORS.iter().cloned().collect();
        set
    };
    static ref SINGLE_OPERATORS_SET: HashSet<&'static str> = {
        let set: HashSet<_> = SINGLE_OPERATORS.iter().cloned().collect();
        set
    };
}

fn is_valid_operator(operator: &str) -> bool {
    VALID_OPERATORS_SET.contains(operator)
}

const SINGLE_OPERATORS: &[&str; 16] = &[
    "sqrt", "sine", "cosine", "tangent", "abs", "floor", "ceiling", "tan", "asin", "acos", "ln",
    "e ^", "sinh", "cosh", "tanh", "atan",
];

fn check_if_certain_operator(operator: &str) -> bool {
    SINGLE_OPERATORS_SET.contains(operator)
}

fn get_second_thing(operator: &str) -> f64 {
    if check_if_certain_operator(operator) {
        69.0
    } else {
        parse_f64(&read_input("Enter the second number to operate on:"))
    }
}

fn perform_operation(first_thing: f64, second_thing: f64, operator: &str) -> f64 {
    let angle_radians = first_thing.to_radians();

    match operator {
        "+" => first_thing + second_thing,
        "-" => first_thing - second_thing,
        "*" => first_thing * second_thing,
        "/" => {
            if second_thing.abs() < 1e-9 || first_thing.abs() < 1e-9 {
                println!("Cannot divide by zero.");
                return 0.0;
            }
            first_thing / second_thing
        }
        "^" => first_thing.powf(second_thing),
        "sqrt" => first_thing.sqrt(),
        "sine" => angle_radians.sin(),
        "cosine" => angle_radians.cos(),
        "tangent" => angle_radians.tan(),
        "abs" => first_thing.abs(),
        "floor" => first_thing.floor(),
        "ceiling" => first_thing.ceil(),
        "tan" => angle_radians.tan(),
        "asin" => first_thing.asin().to_degrees(),
        "acos" => first_thing.acos().to_degrees(),
        "ln" => first_thing.ln(),
        "log" => first_thing.log(second_thing),
        "e ^" => first_thing.exp(),
        "sinh" => first_thing.sinh(),
        "cosh" => first_thing.cosh(),
        "tanh" => first_thing.tanh(),
        "atan2" => first_thing.atan2(second_thing).to_degrees(),
        "atan" => first_thing.atan().to_degrees(),
        _ => {
            println!("The string is not a valid operator.");
            0.0
        }
    }
}

fn main() {
    let first_thing = parse_f64(&read_input("Enter the first number to operate on:"));
    println!("Current calculation: {}", first_thing);

    println!("Enter your operator");
    println!("It can be anything that is:");
    println!("{}", join_valid_operators());
    let binding = read_input("Please enter any of the operators listed above:");
    let operator = binding.trim();

    if !is_valid_operator(operator) {
        println!("The string is not a valid operator.");
        return;
    }

    println!("Current calculation: {} {}", first_thing, operator);

    let second_thing = get_second_thing(operator);

    let result = perform_operation(first_thing, second_thing, operator);

    if !check_if_certain_operator(operator) {
        println!(
            "Result: {} {} {} = {}",
            first_thing, operator, second_thing, result
        );
    } else {
        println!("Result: {} of {} = {}", operator, first_thing, result);
    }
}
