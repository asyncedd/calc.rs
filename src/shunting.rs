pub fn is_not_shunting_yard(expression: &str) -> bool {
    let mut operator_stack: Vec<char> = Vec::new();

    for c in expression.chars() {
        match c {
            '0'..='9' | 'a'..='z' => continue,
            '+' | '-' | '*' | '/' => {
                if operator_stack.is_empty() {
                    operator_stack.push(c);
                } else {
                    let top_operator = *operator_stack.last().unwrap();
                    if precedence(c) > precedence(top_operator)
                        || associativity(top_operator) == Associativity::RightToLeft
                    {
                        operator_stack.push(c);
                    } else {
                        return false;
                    }
                }
            }
            '(' => operator_stack.push(c),
            ')' => {
                if operator_stack.is_empty() || !operator_stack.contains(&'(') {
                    return false;
                }

                while let Some(top_operator) = operator_stack.pop() {
                    if top_operator == '(' {
                        break;
                    }
                }
            }
            _ => return false,
        }
    }

    operator_stack.is_empty() && !operator_stack.contains(&'(')
}

pub fn precedence(operator: char) -> u8 {
    match operator {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

#[derive(PartialEq)]
pub enum Associativity {
    LeftToRight,
    RightToLeft,
}

pub fn associativity(operator: char) -> Associativity {
    match operator {
        '+' | '-' | '*' | '/' => Associativity::LeftToRight,
        _ => Associativity::LeftToRight,
    }
}
