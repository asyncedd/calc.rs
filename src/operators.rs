// operators.rs
use lazy_static::lazy_static;
use std::collections::HashSet;

const VALID_OPERATORS: &[&str; 27] = &[
    "+",
    "-",
    "*",
    "/",
    "^",
    "sqrt",
    "sine",
    "cosine",
    "tangent",
    "abs",
    "floor",
    "ceiling",
    "tan",
    "asin",
    "acos",
    "ln",
    "log",
    "e ^",
    "sinh",
    "cosh",
    "tanh",
    "atan2",
    "atan",
    "primes",
    "collatz",
    "perfect",
    "fibonacci",
];

const SINGLE_OPERATORS: &[&str; 19] = &[
    "sqrt",
    "sine",
    "cosine",
    "tangent",
    "abs",
    "floor",
    "ceiling",
    "tan",
    "asin",
    "acos",
    "ln",
    "e ^",
    "sinh",
    "cosh",
    "tanh",
    "atan",
    "collatz",
    "perfect",
    "fibonacci",
];

const GEN_OPERATORS: &[&str; 4] = &["primes", "collatz", "perfect", "fibonacci"];

lazy_static! {
    pub static ref VALID_OPERATORS_SET: HashSet<&'static str> = {
        let set: HashSet<_> = VALID_OPERATORS.iter().cloned().collect();
        set
    };
    pub static ref SINGLE_OPERATORS_SET: HashSet<&'static str> = {
        let set: HashSet<_> = SINGLE_OPERATORS.iter().cloned().collect();
        set
    };
    pub static ref GEN_OPERATORS_SET: HashSet<&'static str> = {
        let set: HashSet<_> = GEN_OPERATORS.iter().cloned().collect();
        set
    };
}

pub fn join_valid_operators() -> String {
    VALID_OPERATORS.join(", ")
}

pub fn is_valid_operator(operator: &str) -> bool {
    VALID_OPERATORS_SET.contains(operator)
}

pub fn check_if_certain_operator(operator: &str) -> bool {
    SINGLE_OPERATORS_SET.contains(operator)
}

pub fn check_if_gen_operator(operator: &str) -> bool {
    GEN_OPERATORS_SET.contains(operator)
}
