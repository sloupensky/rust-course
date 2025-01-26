use slug::slugify;
use std::env;
use std::io::stdin;

fn modify_string(string: String, modification_type: &str) -> String {
    match modification_type {
        "lowercase" => string.to_lowercase(),
        "uppercase" => string.to_uppercase(),
        "no-spaces" => string.replace(" ", ""),
        "slugify" => slugify(string),
        _ => panic!("Unknown slugify {}", string),
    }
}

fn main() {
    let mut entered_string = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!(
            "There is only one allowed argument, you entered {}",
            args.len() - 1
        );
    } else if args.len() == 2 && args[1].is_empty() {
        panic!("Argument cannot be empty");
    } else {
        println!("Entered cli argument {}", args[1])
    }

    println!("Enter your input: ");

    stdin().read_line(&mut entered_string).unwrap();

    println!("Modified string is: {}", modify_string(entered_string, args[1].as_str()))
}
