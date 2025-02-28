use std::io::stdin;
use std::error::Error;
use std::str::FromStr;

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

impl FromStr for Mode {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "client" => Ok(Mode::Client),
            "server" => Ok(Mode::Server),
            _ => Err(format!("Unknown mode provided {}", s).into()),
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

pub fn get_mode() -> Result<Mode, Box<dyn Error>> {
    let stdin = stdin();
    let mut mode = String::new();

    println!("Enter in which mode you want to start (client or server):");
    stdin.read_line(&mut mode)?;

    Mode::from_str(mode.as_str())
}

pub fn get_operation_type() -> Result<InputOperationType, Box<dyn Error>> {
    let stdin = stdin();
    let mut input_mode = String::new();

    println!("Enter which operation type you want to use (image/file/quit) or any text:");
    stdin.read_line(&mut input_mode)?;
    let operation_type = Operation::from_str(input_mode.as_str())?;

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

pub fn get_address() -> Result<String, Box<dyn Error>> {
    let stdin = stdin();
    let mut address = String::new();

    println!("Enter address and port (default localhost:1111):");
    stdin.read_line(&mut address)?;

    if address.trim().is_empty() {
        address = DEFAULT_ADDRESS.to_string();
    }

    Ok(address.trim().to_string())
}