mod input;
use input::read_input;
use lazy_static::lazy_static;
use std::collections::HashSet;

fn parse_f64(string: &str) -> f64 {
    string.parse().expect("Invalid number")
}

const VALID_OPERATORS: &[&str; 26] = &[
    "+", "-", "*", "/", "^", "sqrt", "sine", "cosine", "tangent", "abs", "floor", "ceiling", "tan",
    "asin", "acos", "ln", "log", "e ^", "sinh", "cosh", "tanh", "atan2", "atan", "primes",
    "collatz", "perfect",
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
    static ref GEN_OPERATORS_SET: HashSet<&'static str> = {
        let set: HashSet<_> = GEN_OPERATORS.iter().cloned().collect();
        set
    };
}

fn is_valid_operator(operator: &str) -> bool {
    VALID_OPERATORS_SET.contains(operator)
}

const SINGLE_OPERATORS: &[&str; 18] = &[
    "sqrt", "sine", "cosine", "tangent", "abs", "floor", "ceiling", "tan", "asin", "acos", "ln",
    "e ^", "sinh", "cosh", "tanh", "atan", "collatz", "perfect",
];

const GEN_OPERATORS: &[&str; 3] = &["primes", "collatz", "perfect"];

fn check_if_certain_operator(operator: &str) -> bool {
    SINGLE_OPERATORS_SET.contains(operator)
}

fn check_if_gen_operator(operator: &str) -> bool {
    GEN_OPERATORS_SET.contains(operator)
}

fn get_second_thing(operator: &str) -> f64 {
    if check_if_certain_operator(operator) {
        69.0
    } else {
        parse_f64(&read_input("Enter the second number to operate on:"))
    }
}

fn gen_primes(first_thing: usize, second_thing: usize) {
    println!("\nHere are your primes:");
    let primes = generate_primes(second_thing);

    for prime in primes.iter().filter(|&p| *p >= first_thing) {
        println!("{}", prime);
    }
}

fn generate_primes(n: usize) -> Vec<usize> {
    let mut sieve = vec![true; n + 1];
    let mut primes = Vec::new();

    sieve[0] = false;
    sieve[1] = false;

    for p in 2..=(n as f64).sqrt() as usize {
        if sieve[p] {
            primes.push(p);
            for i in (p * p..=n).step_by(p) {
                sieve[i] = false;
            }
        }
    }

    for p in ((n as f64).sqrt() as usize + 1)..=n {
        if sieve[p] {
            primes.push(p);
        }
    }

    primes
}

fn collatz_sequence(n: f64) {
    println!("{}", n);
    if n == 1.0 {
        return;
    }

    if n % 2.0 == 0.0 {
        collatz_sequence(n / 2.0);
    } else {
        collatz_sequence(3.0 * n + 1.0);
    }
}

fn perform_operation_gen(first_thing: f64, second_thing: f64, operator: &str) {
    match operator {
        "primes" => gen_primes(first_thing as usize, second_thing as usize),
        "collatz" => collatz_sequence(first_thing),
        "perfect" => print_perfect_numbers(first_thing),
        &_ => todo!(),
    }
}

fn find_perfect_numbers(count: usize) -> Vec<u64> {
    let mut perfect_numbers = Vec::new();
    let mut number = 2;
    let mut primes = vec![2]; // Store prime numbers to generate perfect numbers

    while perfect_numbers.len() < count {
        let sqrt = (number as f64).sqrt() as u64;
        let mut sum_of_divisors = 1;

        // Check divisibility only up to the square root of number
        for divisor in 2..=sqrt {
            if number % divisor == 0 {
                sum_of_divisors += divisor;

                // Add the corresponding divisor greater than the square root
                if divisor != number / divisor {
                    sum_of_divisors += number / divisor;
                }
            }
        }

        if sum_of_divisors == number {
            perfect_numbers.push(number);
        }

        number += 1;

        // Generate prime numbers using the Sieve of Eratosthenes algorithm
        if primes.iter().all(|prime| number % prime != 0) {
            primes.push(number);
        }
    }

    perfect_numbers
}

fn print_perfect_numbers(count: f64) {
    let perfect_numbers = find_perfect_numbers(count as usize);

    println!("First {} perfect numbers:", count);
    for perfect_number in perfect_numbers {
        println!("{}", perfect_number);
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

    if !check_if_gen_operator(operator) {
        let result = perform_operation(first_thing, second_thing, operator);

        if !check_if_certain_operator(operator) {
            println!(
                "Result: {} {} {} = {}",
                first_thing, operator, second_thing, result
            );
        } else {
            println!("Result: {} of {} = {}", operator, first_thing, result);
        }
    } else {
        perform_operation_gen(first_thing, second_thing, operator)
    }
}
