// Use the "io" from the standard library
use std::io;

// A fancy new function
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

// Predefine the operators
const VALID_OPERATORS: [&str; 9] = ["+", "-", "*", "/", "^", "sqrt", "sine", "cosine", "tangent"];

// Checks if the operator is a operator that is supported
fn is_valid_operator(operator: &str) -> bool {
    VALID_OPERATORS.contains(&operator)
}

// Operators that are single
const SINGLE_OPERATORS: [&str; 4] = ["sqrt", "sine", "cosine", "tangent"];

// Check if the operators is one defined above
fn check_if_certain_operator(operator: &str) -> bool {
    SINGLE_OPERATORS.contains(&operator)
}

fn main() {
    // Declare these variables as strings.
    // for both the first and second things, it'll be converted to a 64-bit integer later
    let first_thing = parse_f64(&read_input("Enter the first number to operate on:"));

    // Print the current calculation
    println!("Current calculation: {}", first_thing);

    // Get the operator
    let operator = read_input("Enter your operator\nIt can be anything that is: \n+, -, *, /, ^, sqrt, sine, cosine, tangent\nPlease enter any of the opreators listed above:");
    // Trim whitespaces from the operator
    let operator = operator.trim();

    // Use the `is_valid_operator` function to check if it is.
    // If it returns anything that is not true, error out and return
    if !is_valid_operator(operator) {
        println!("The string is not a valid operator.");
        return;
    }

    // Print the current calculation
    println!("Current calculation: {} {}", first_thing, operator);

    // Define the second_thing varaible before hand.
    let second_thing: f64;

    // If the opertor isn't "sqrt" then, ask for user input
    if !check_if_certain_operator(operator) {
        second_thing = parse_f64(&read_input("Enter the second number to operate on:"));

        // Print the current calculation again
        println!(
            "Current calculation: {} {} {}",
            first_thing, operator, second_thing
        );
    } else {
        // otherwise, define a dummy variable
        second_thing = 69.0
    }

    // If it matches a:
    let result = match operator {
        // "+" add
        "+" => first_thing + second_thing,
        // "-" substract
        "-" => first_thing - second_thing,
        // "*" multiply
        "*" => first_thing * second_thing,
        // "/" divide
        "/" => {
            // Check if the second_thing is near zero
            if second_thing.abs() < 1e-9 || first_thing.abs() < 1e-9 {
                println!("Cannot divide by zero.");
                return;
            }
            first_thing / second_thing
        }
        "^" => first_thing.powf(second_thing),
        "sqrt" => first_thing.sqrt(),
        "sine" => {
            let angle_radians = first_thing.to_radians(); // Convert degrees to radians

            angle_radians.sin() // Calculate the sine of the angle
        }
        "cosine" => {
            let angle_radians = first_thing.to_radians(); // Convert degrees to radians

            angle_radians.cos() // Calculate the sine of the angle
        }
        "tangent" => {
            let angle_radians = first_thing.to_radians(); // Convert degrees to radians

            angle_radians.tan() // Calculate the sine of the angle
        }
        // If it doesn't match any of these, prints an error
        _ => {
            println!("The string is not a valid operator.");
            return;
        }
    };

    // Print the result
    if !check_if_certain_operator(operator) {
        println!(
            "Result: {} {} {} = {}",
            first_thing, operator, second_thing, result
        );
    } else {
        println!("Result: {} {} = {}", first_thing, operator, result);
    }
}
