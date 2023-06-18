// input.rs
use colored::Colorize;
use std::io;
use std::io::Write;

pub fn read_input(message: &str) -> String {
    // If the message isn't "" print the message
    if message != "" {
        println!("{}", message);
    }
    // Print ">" in a bright_black asking for user input
    //
    // eg.
    //
    // [MESSAGE]
    // > [USER_INPUT]
    //
    print!("{} ", "> ".bright_black());

    // Flush the output stram to ensure any buffered data is flushed out
    // Try to extract the data from Result
    io::stdout().flush().unwrap();

    // Create a new string to write data to.
    let mut input = String::new();
    io::stdin()
        // Read line from `input` (which is mutable)
        .read_line(&mut input)
        // If it isn't error out.
        .expect("Failed to read line");
    // Return the trimmed input and make it into a String
    input.trim().to_string()
}
