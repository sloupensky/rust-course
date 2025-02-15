use std::error::Error;
use std::io::{Lines, StdinLock};
use std::io::stdin;

pub struct InputOperation {
    pub command: String,
    pub modification_type: String,
}


pub enum InteractiveMode {
    FullyInteractive,
    TypeProvided,
    NonInteractive,
}

pub fn handle_input(arguments: &Vec<String>) -> Result<InputOperation, Box<dyn Error>> {
    let interactive_result = self::get_interaction(&arguments);

    match interactive_result {
        Ok(InteractiveMode::FullyInteractive) => {
            self::handle_fully_interactive()
        },
        Ok(InteractiveMode::TypeProvided) => {
            self::handle_type_provided(&arguments)
        },
        Ok(InteractiveMode::NonInteractive) => {
            self::handle_non_interactive(&arguments)
        },
        Err(error) => {
            Err(error.into())
        }
    }
}

fn handle_fully_interactive() -> Result<InputOperation, Box<dyn Error>> {
    let mut modification_type = String::new();

    println!("Enter one of modification types (csv, uppercase, lowercase, slugify):");
    stdin().read_line(&mut modification_type).unwrap();

    println!("Enter your input or file path(csv.csv) when csv parameter used (ctrl + D to accept input): ");
    match self::transform_input_into_string(stdin().lines()) {
        Ok(command) => {
            Ok(InputOperation{command, modification_type})
        },
        Err(error) => Err(error.into())
    }
}

fn handle_type_provided(arguments: &Vec<String>) -> Result<InputOperation, Box<dyn Error>> {
    let modified_string = self::get_modification_type(&arguments);

    match modified_string {
        Ok(modification_type) => {
            println!("Enter your input or file path when csv parameter used (ctrl + D to accept input): ");
            match self::transform_input_into_string(stdin().lines()) {
                Ok(command) => {
                    Ok(InputOperation{command, modification_type})
                },
                Err(error) => Err(error.into()),
            }
        },
        Err(error) => Err(error.into()),
    }
}
fn handle_non_interactive(arguments: &Vec<String>) -> Result<InputOperation, Box<dyn Error>> {
    match self::get_input_operation(&arguments) {
        Ok(result) => {
            Ok(InputOperation{command: result.command, modification_type: result.modification_type})
        },
        Err(error) => Err(error.into()),
    }
}
fn get_input_operation(arguments: &Vec<String>) -> Result<InputOperation, Box<dyn Error>> {
    match self::get_interaction(arguments) {
        Ok(InteractiveMode::NonInteractive) => {
            Ok(InputOperation{command: arguments[2].clone(), modification_type: arguments[1].clone() })
        },
        _ => Err(format!("Unable to determine type from provided arguments").into())
    }
}
fn get_interaction(arguments: &Vec<String>) -> Result<InteractiveMode, Box<dyn Error>> {
    match arguments.len() {
        0..=1 => Ok(InteractiveMode::FullyInteractive),
        2 => Ok(InteractiveMode::TypeProvided),
        3.. => Ok(InteractiveMode::NonInteractive),
    }
}

fn get_modification_type(arguments: &Vec<String>) -> Result<String, Box<dyn Error>> {
    match self::get_interaction(arguments) {
        Ok(InteractiveMode::TypeProvided) => Ok(arguments[1].clone()),
        _ => Err(format!("Unable to determine type from provided arguments").into())
    }
}
fn transform_input_into_string(lines: Lines<StdinLock<'static>>) -> Result<String, Box<dyn Error>> {
    let string = self::concatenate_lines(lines)?;

    if string.len() == 1 {
        return Err("String cannot be empty".into());
    }

    Ok(string)
}

fn concatenate_lines(lines: Lines<StdinLock<'static>>) -> Result<String, Box<dyn Error>> {
    let string_lines:Vec<String> = lines.map(|line| line.unwrap()).collect();

    Ok(string_lines.join("\n"))
}
