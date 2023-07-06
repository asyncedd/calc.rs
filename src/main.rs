mod input;
mod math;
mod operators;
mod test;

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

fn perform_operation(first_thing: f64, second_thing: f64, operator: &str) -> Result<f64, String> {
    let angle_radians = first_thing.to_radians();

    match operator {
        "+" => Ok(first_thing + second_thing),
        "-" => Ok(first_thing - second_thing),
        "*" => Ok(first_thing * second_thing),
        "/" => {
            if second_thing == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(first_thing / second_thing)
            }
        }
        "^" => Ok(first_thing.powf(second_thing)),
        "%" => Ok(first_thing % second_thing),
        "sqrt" => Ok(first_thing.sqrt()),
        "sine" => Ok(angle_radians.sin()),
        "cosine" => Ok(angle_radians.cos()),
        "tangent" => Ok(angle_radians.tan()),
        "abs" => Ok(first_thing.abs()),
        "floor" => Ok(first_thing.floor()),
        "ceiling" => Ok(first_thing.ceil()),
        "tan" => Ok(angle_radians.tan()),
        "asin" => Ok(first_thing.asin()),
        "acos" => Ok(first_thing.acos()),
        "ln" => Ok(first_thing.ln()),
        "log" => Ok(first_thing.log(second_thing)),
        "e ^" => Ok(first_thing.exp()),
        "sinh" => Ok(first_thing.sinh()),
        "cosh" => Ok(first_thing.cosh()),
        "tanh" => Ok(first_thing.tanh()),
        "atan2" => Ok(first_thing.atan2(second_thing)),
        "atan" => Ok(first_thing.atan()),
        _ => Err("The string is not a valid operator.".to_string()),
    }
}

fn read_number_input(message: &str) -> Result<f64, String> {
    let input = read_input(message);
    Ok(eval(&input)
        .map_or_else(|_| parse_f64(&input), |result| parse_f64(&result.to_string())))
}

fn main() {
    let first_thing = match read_number_input("Enter the first number to operate on, or an expression:") {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err.red());
            return;
        }
    };

    println!(
        "Please enter the operator for your calculation\n{}",
        join_valid_operators().green()
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
        match read_number_input("Enter the second number to operate on:") {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err.red());
                return;
            }
        }
    };

    if !check_if_gen_operator(operator) {
        match perform_operation(first_thing, second_thing, operator) {
            Ok(result) => {
                if !check_if_certain_operator(operator) {
                    println!("{} {} {} {} = {}", "Result:".green(), first_thing, operator, second_thing, result);
                } else {
                    println!("{}: {} of {} = {}", "Result".green(), operator, first_thing, result);
                }
            }
            Err(err) => {
                println!("{}", err.red());
            }
        }
    } else {
        perform_operation_gen(first_thing, second_thing, operator)
    }
}
