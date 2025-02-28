use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use image::ImageReader;
use chrono::prelude::*;

pub fn read_file_to_vec(path: &String) -> Result<Vec<u8>, Box<dyn Error>> {
    let data = fs::read(path)?;

    Ok(data)
}


pub fn save_and_convert_image(file: &str, content: &[u8]) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file);
    let filename = path.file_name().unwrap().to_str().unwrap();
    let filepath = format!("./images/{}", filename);
    let created_file = File::create(filepath.clone());
    let current_timestamp = Local::now().timestamp();

    match created_file {
        Ok(mut file) => {
            file.write_all(content)?;
            let img = ImageReader::open(filepath.clone())?.decode()?;

            img.save(format!("./images/{}.png", current_timestamp))?;
            fs::remove_file(filepath.clone())?;
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn save_file(file: &str, content: &[u8]) -> Result<(), Box<dyn Error>> {
    let path = Path::new(file);
    let filename = path.file_name().unwrap().to_str().unwrap();
    let created_file = File::create(format!("./files/{}", filename));

    match created_file {
        Ok(mut file) => Ok(file.write_all(content)?),
        Err(e) => Err(Box::new(e)),
    }
}