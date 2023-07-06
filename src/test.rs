use crate::*;

#[test]
fn test_operators_module() {
    // Test is_valid_operator function
    assert_eq!(is_valid_operator("+"), true);
    assert_eq!(is_valid_operator("invalid"), false);

    // Test check_if_certain_operator function
    assert_eq!(check_if_certain_operator("sqrt"), true);
    assert_eq!(check_if_certain_operator("+"), false);

    // Test check_if_gen_operator function
    assert_eq!(check_if_gen_operator("primes"), true);
    assert_eq!(check_if_gen_operator("+"), false);
}


#[test]
fn test_math_module() {
    // Test parse_f64 function
    let number_str = "3.14";
    let parsed_number = parse_f64(number_str);
    assert_eq!(parsed_number, 3.14);

    // Test gen_primes function
    let first_thing = 2;
    let second_thing = 20;
    gen_primes(first_thing, second_thing);

    // Test collatz_sequence function
    let n = 10.0;
    collatz_sequence(n);

    // Test get_fibonacci function
    let n = 10;
    get_fibonacci(n);
}
