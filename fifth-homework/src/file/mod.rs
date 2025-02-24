use std::fs;
use std::error::Error;

pub fn read_file_to_vec(path: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    let data = fs::read(path)?;

    Ok(data)
}