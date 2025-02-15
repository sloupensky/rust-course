mod string_operation;
mod input_operation;
use std::env;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let arguments:Vec<String> = env::args().collect();

    thread::Builder::new().name("input-thread".into()).spawn(move || {
        match input_operation::handle_input(&arguments) {
            Ok(input) => tx.send((input.command, input.modification_type)).unwrap(),
            Err(err) => {
                eprintln!("Error: {:?}", err);
                drop(tx);
            }
        }
    }).unwrap();

    while let Ok((command, modification_type)) = rx.recv() {
        match string_operation::modify_string(command, &modification_type) {
            Ok(content) => {
                println!("Modified string is:");
                println!("{}", content)
            },
            Err(error) => {
                eprintln!("{}", error)
            }
        };
    }
}
