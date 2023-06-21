mod input;
mod math;
mod operators;

use colored::Colorize;
use eval::eval;

use input::*;
use math::*;
use operators::*;

fn perform_operation_gen(first_thing: f64, second_thing: f64, operator: &str) {
    match operator {
        "primes" => gen_primes(first_thing as usize, second_thing as usize),
        "collatz" => collatz_sequence(first_thing),
        "perfect" => print_perfect_numbers(first_thing),
        "fibonacci" => get_fibonacci(first_thing as u32),
        &_ => todo!(),
    }
}

fn perform_operation(first_thing: f64, second_thing: f64, operator: &str) -> f64 {
    let angle_radians = first_thing.to_radians();

    match operator {
        "+" => first_thing + second_thing,
        "-" => first_thing - second_thing,
        "*" => first_thing * second_thing,
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
        "^" => first_thing.powf(second_thing),
        "%" => first_thing % second_thing,
        "sqrt" => first_thing.sqrt(),
        "sine" => angle_radians.sin(),
        "cosine" => angle_radians.cos(),
        "tangent" => angle_radians.tan(),
        "abs" => first_thing.abs(),
        "floor" => first_thing.floor(),
        "ceiling" => first_thing.ceil(),
        "tan" => angle_radians.tan(),
        "asin" => first_thing.asin(),
        "acos" => first_thing.acos(),
        "ln" => first_thing.ln(),
        "log" => first_thing.log(second_thing),
        "e ^" => first_thing.exp(),
        "sinh" => first_thing.sinh(),
        "cosh" => first_thing.cosh(),
        "tanh" => first_thing.tanh(),
        "atan2" => first_thing.atan2(second_thing),
        "atan" => first_thing.atan(),
        _ => {
            println!("{}", "The string is not a valid operator.".red());
            // Return 0 as a dummy value
            0.0
        }
    }
}

// the main function
fn main() {
    // Ask for the first number and assign it to the first_thing variable
    // read_input accepts a message and returns a String
    // the String that is returned is the user input
    //
    // eg.
    //
    // Enter the first number to operate on, or an expression:
    // > [USER_INPUT]
    //
    // Then, [USER_INPUT] would be returned
    let first_thing = read_input("Enter the first number to operate on, or an expression:");
    let first_thing = if let Ok(result) = eval(&first_thing) {
        // If it's a valid expression, use the evaluated result
        parse_f64(&result.to_string())
    } else {
        // Otherwise, treat it as a number
        parse_f64(&first_thing)
    };
    // Check if it's not a shunting yard
    // If it is, go ahead and parse it to f64
    // Ask for the operator (+, -, *, / etc..)
    //
    // eg.
    //
    // Please enter the operator for your calculation
    // (+, -, *, / ...) (In reality it'll have more listed.)
    println!(
        "Please enter the operator for your calculation\n({})",
        join_valid_operators().green(),
    );
    // Since, we don't need to ask for anything just input ""
    let binding = read_input("");
    // trim unneeded whitespaces
    let operator = binding.trim();

    if !is_valid_operator(operator) {
        println!("{}", "The string is not a valid operator.".red());
        return;
    }

    let second_thing = if check_if_certain_operator(operator) {
        69.0
    } else {
        let second_input = read_input("Enter the second number to operate on:");
        match eval(second_input.as_str()) {
            Ok(value) => {
                let result = parse_f64(&value.to_string());
                result
            }
            Err(_err) => parse_f64(&second_input),
        }
    };

    // Check if it isn't a gen_operator
    // (primes, etc..)
    if !check_if_gen_operator(operator) {
        // if it isn't go ahead and perform_operation normally
        let result = perform_operation(first_thing, second_thing, operator);

        // If it isn't an opreator that needs two numbers to operate on, print it as below:
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
            // Elseif it's an operator that requires two numbers to operte on, print it as
            // below:
            println!(
                "{}: {} of {} = {}",
                "Result".green(),
                operator,
                first_thing,
                result
            );
        }
    } else {
        // Otherwise the execute operator_gen.
        perform_operation_gen(first_thing, second_thing, operator)
    }
}
