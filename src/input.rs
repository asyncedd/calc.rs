// input.rs
use std::io;
use std::io::Write;

pub fn read_input(message: &str) -> String {
    if message != "" {
        println!("{}", message);
    }
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
