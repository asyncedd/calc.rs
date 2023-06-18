// Import some modules.
// "input" is for input-based functions
mod input;
// "math" is for mathematical functions
mod math;
// "operators" is for opeerator functions
mod operators;
// "shunting" is for shunting yard functions
mod shunting;

// Input the "colored" crate
// This is for colorizing the strings.
use colored::Colorize;
// Input the "eval" crate
// This is for evaling mathematical expressions
use eval::eval;

// This is for importing the "input" module.
use input::*;
// This is for importing the "math" module.
use math::*;
// This is for importing the "operator" module.
use operators::*;
// This is for importing the "shunting yard" module.
use shunting::*;

// Get the second thing function.
// This is really just a small wrapper that returns an integer from the "read_input" function.
// If it doesn't get one, just return a fallback integer.
// It isn't really important on what it is.
fn get_second_thing(operator: &str) -> f64 {
    if check_if_certain_operator(operator) {
        69.0
    } else {
        parse_f64(&read_input("Enter the second number to operate on:"))
    }
}

// Perform operator that is related to generating operations
// The reasoning that it isn't inside the "perform_operation" function is because, it's usually
// returns an array etc...
fn perform_operation_gen(first_thing: f64, second_thing: f64, operator: &str) {
    // It'll match if it isn't a "primes", "collatz", "perfect" and "fibonacci" operator
    match operator {
        // "gen_primes" needs "usize" so, we'll input both first_thing and second_thing (f64) as
        // You can "make" f64's (64-bit floating numbers) into usize
        "primes" => gen_primes(first_thing as usize, second_thing as usize),
        "collatz" => collatz_sequence(first_thing),
        "perfect" => print_perfect_numbers(first_thing),
        "fibonacci" => get_fibonacci(first_thing as u32),
        &_ => todo!(),
    }
}

fn perform_operation(first_thing: f64, second_thing: f64, operator: &str) -> f64 {
    // make the f64 first_thing to radians.
    // Because, some operators need them.
    let angle_radians = first_thing.to_radians();

    match operator {
        // addition
        // This will simply add first_thing to second_thing.
        "+" => first_thing + second_thing,
        // subtraction
        // This will subtract second_thing from first_thing
        "-" => first_thing - second_thing,
        // multiplication
        // This will multiply first_thing with second_thing
        "*" => first_thing * second_thing,
        // division
        // This will, if first_thing and second_thing aren't zero or near zero, divide second_thing
        // from first_thing.
        "/" => {
            // Set the epsilon variable to 1e-9 (which is a magic number).
            // 1e-9 is a very small number thus it'll effectively remove all numbers that are 0 or
            // VERY near 0. (0.000000001)
            let epsilon = 1e-9;
            // If either first or second thing is smaller than 1e-9
            if second_thing.abs() < epsilon || first_thing.abs() < epsilon {
                println!("{}", "Cannot divide by zero.".red());
                // Return 0 as a dummy value
                return 0.0;
            }
            // otherwise continue
            first_thing / second_thing
        }
        // calculation for power of.
        "^" => first_thing.powf(second_thing),
        // calculation for square roots
        // It'll get the square root of first_thing.
        // second_thing is irrelevant since, what are we going to do with it? 🤔
        "sqrt" => first_thing.sqrt(),
        // calculate the sine of first_thing (which is a radian now)
        "sine" => angle_radians.sin(),
        // calculate the cosine of first_thing
        "cosine" => angle_radians.cos(),
        // calculate the tangent of first_thing
        "tangent" => angle_radians.tan(),
        // calculate the absolute value of first_thing
        "abs" => first_thing.abs(),
        // find the floor of first_thing
        "floor" => first_thing.floor(),
        // find the ceiling of first_thing
        "ceiling" => first_thing.ceil(),
        // calculate the tangent of the angle radians of first_thing
        "tan" => angle_radians.tan(),
        // calculate the arcsine of first_thing
        "asin" => first_thing.asin(),
        // calculate the arccosine of first_thing
        "acos" => first_thing.acos(),
        "ln" => first_thing.ln(),
        // calculate the logarithm of first_thing with base second_thing
        "log" => first_thing.log(second_thing),
        // calculate e raised to the power of first_thing
        "e ^" => first_thing.exp(),
        "sinh" => first_thing.sinh(),
        "cosh" => first_thing.cosh(),
        "tanh" => first_thing.tanh(),
        // calculate the arctangent of first_thing divided by second_thing
        "atan2" => first_thing.atan2(second_thing),
        // calculate the arctangent of first_thing
        "atan" => first_thing.atan(),
        _ => {
            println!("{}", "The string is not a valid operator.".red());
            // Return 0 as a dummy value
            0.0
        }
    }
}

fn main() {
    let first_thing = read_input("Enter the first number to operate on, or an expression:");
    if is_not_shunting_yard(first_thing.as_str()) {
        let first_thing = parse_f64(&first_thing.to_string());
        println!(
            "Please enter the operator for your calculation\n({})",
            join_valid_operators().green(),
        );
        let binding = read_input("");
        let operator = binding.trim();

        if !is_valid_operator(operator) {
            println!("{}", "The string is not a valid operator.".red());
            return;
        }

        let second_thing = get_second_thing(operator);

        if !check_if_gen_operator(operator) {
            let result = perform_operation(first_thing, second_thing, operator);

            if !check_if_certain_operator(operator) {
                println!(
                    "{} {} {} {} = {}",
                    "Result:".green(),
                    first_thing,
                    operator,
                    second_thing,
                    result
                );
            } else {
                println!(
                    "{}: {} of {} = {}",
                    "Result".green(),
                    operator,
                    first_thing,
                    result
                );
            }
        } else {
            perform_operation_gen(first_thing, second_thing, operator)
        }
    } else {
        if let Ok(result) = eval(first_thing.as_str()) {
            println!("{}: {}", "Result".green(), result);
        } else {
            println!("{}", "Invalid expression".red());
        }
    }
}
