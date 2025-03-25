use std::io::stdin;
use std::error::Error;
use std::str::FromStr;
use thiserror::Error;

const DEFAULT_ADDRESS: &str = "localhost:1111";
pub enum Mode {
    Server,
    Client,
}

pub enum Operation {
    Image,
    File,
    Text,
    Quit
}

pub struct InputOperationType {
    pub operation: Operation,
    pub data: String
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Unknown mode provided: `{0}`")]
    UnknownMode(String),
    #[error("Provided address `{0}` is not valid")]
    InvalidAddress(String),
    #[error("Unable to read line from std input")]
    ReadLineError(#[from] std::io::Error),
    #[error("Provided operation mode `{0}` is not valid")]
    InvalidOperationMode(String),
    #[error("Empty user provided")]
    EmptyUser,
}

impl FromStr for Mode {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, InputError> {
        match s.trim() {
            "client" => Ok(Mode::Client),
            "server" => Ok(Mode::Server),
            _ => Err(InputError::UnknownMode(s.to_string())),
        }
    }
}

impl FromStr for Operation {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "image" => Ok(Operation::Image),
            "file" => Ok(Operation::File),
            "quit" => Ok(Operation::Quit),
            _ => Ok(Operation::Text),
        }
    }
}

pub fn get_mode() -> Result<Mode, InputError> {
    let stdin = stdin();
    let mut mode = String::new();

    println!("Enter in which mode you want to start (client or server):");
    stdin.read_line(&mut mode).map_err(|err| InputError::ReadLineError(err))?;

    Mode::from_str(mode.as_str()).map_err(|err| InputError::UnknownMode(err.to_string()))
}

pub fn get_operation_type() -> Result<InputOperationType, InputError> {
    let stdin = stdin();
    let mut input_mode = String::new();

    println!("Enter which operation type you want to use (image/file/quit) or any text:");
    stdin.read_line(&mut input_mode)?;
    let operation_type = Operation::from_str(input_mode.as_str()).map_err(|err| InputError::InvalidAddress(err.to_string()))?;

    match operation_type {
        Operation::Image|Operation::File => {
            let mut path = String::new();

            println!("Enter path:");
            stdin.read_line(&mut path)?;

            Ok(InputOperationType{operation: operation_type, data: path.trim().to_string()})
        },
        Operation::Text => {
            Ok(InputOperationType{operation: operation_type, data: input_mode.to_string()})
        },
        Operation::Quit => {
            Ok(InputOperationType{operation: operation_type, data: String::new()})
        }
    }
}

pub fn get_address() -> Result<String, InputError> {
    let stdin = stdin();
    let mut address = String::new();

    println!("Enter address and port (default localhost:1111):");
    stdin.read_line(&mut address)?;

    if address.trim().is_empty() {
        address = DEFAULT_ADDRESS.to_string();
    }

    is_valid_address(&address)?;

    Ok(address.trim().to_string())
}

pub fn get_user() -> Result<String, InputError> {
    let stdin = stdin();
    let mut user = String::new();

    println!("Enter user:");
    stdin.read_line(&mut user)?;

    if user.trim().is_empty() {
        return Err(InputError::EmptyUser);
    }

    Ok(user.trim().to_string())
}

fn is_valid_address(address: &String) -> Result<bool, InputError> {
    let split_address = address.split(":").collect::<Vec<&str>>();

    if split_address.len() == 2 {
        Ok(true)
    } else {
        Err(InputError::InvalidAddress(address.to_owned()).into())
    }
}