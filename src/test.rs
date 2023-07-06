#[cfg(test)]
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
}

#[test]
fn test_perform_operation_addition() {
    assert_eq!(perform_operation(2.0, 3.0, "+"), Ok(5.0));
}

#[test]
fn test_perform_operation_subtraction() {
    assert_eq!(perform_operation(5.0, 2.0, "-"), Ok(3.0));
}
