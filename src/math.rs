// math.rs

pub fn parse_f64(string: &str) -> f64 {
    string.parse().expect("Invalid number")
}

pub fn gen_primes(first_thing: usize, second_thing: usize) {
    let primes = generate_primes(second_thing);

    let primes_to_print = primes
        .iter()
        .filter(|&p| *p >= first_thing)
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("\nHere are your primes: {}", primes_to_print);
}

pub fn generate_primes(n: usize) -> Vec<usize> {
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

pub fn collatz_sequence(n: f64) {
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

pub fn find_perfect_numbers(count: usize) -> Vec<u64> {
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

pub fn print_perfect_numbers(count: f64) {
    let perfect_numbers = find_perfect_numbers(count as usize);

    println!("First {} perfect numbers:", count);
    for perfect_number in perfect_numbers {
        println!("{}", perfect_number);
    }
}
