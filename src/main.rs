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
        _ => todo!(),
    }
}

fn perform_operation(first_thing: f64, second_thing: f64, operator: &str) -> f64 {
    let angle_radians = first_thing.to_radians();

    match operator {
        "+" => first_thing + second_thing,
        "-" => first_thing - second_thing,
        "*" => first_thing * second_thing,
        "/" => {
            if second_thing == 0.0 {
                return 0.0;
            }
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
            0.0
        }
    }
}

fn main() {
    let first_thing = read_input("Enter the first number to operate on, or an expression:");
    let first_thing: f64 = if let Ok(result) = eval(&first_thing) {
        parse_f64(&result.to_string())
    } else {
        parse_f64(&first_thing)
    };

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

    let second_thing = if check_if_certain_operator(operator) {
        69.0
    } else {
        let second_input = read_input("Enter the second number to operate on:");
        match eval(second_input.as_str()) {
            Ok(value) => parse_f64(&value.to_string()),
            Err(_) => parse_f64(&second_input),
        }
    };

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
}
