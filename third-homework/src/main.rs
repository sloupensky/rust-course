mod string_operation;
use std::env;
use std::io::stdin;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Enter your input (for csv you need to use separator ; and ctrl + D to accept input): ");

    match string_operation::modify_string(stdin().lines(), args) {
        Ok(content) => {
            println!("Modified string is:");
            println!("{}", content)
        },
        Err(error) => {
            eprintln!("{}", error)
        }
    }
}
