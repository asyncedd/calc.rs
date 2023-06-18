mod input;
mod math;
mod operators;
mod shunting;

use colored::Colorize;
use eval::eval;
use input::read_input;
use math::*;
use operators::*;
use shunting::*;

fn get_second_thing(operator: &str) -> f64 {
    if check_if_certain_operator(operator) {
        69.0
    } else {
        parse_f64(&read_input("Enter the second number to operate on:"))
    }
}

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
            let epsilon = 1e-9;
            if second_thing.abs() < epsilon || first_thing.abs() < epsilon {
                println!("{}", "Cannot divide by zero.".red());
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
