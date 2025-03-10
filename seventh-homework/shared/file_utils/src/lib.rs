use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use image::ImageReader;
use chrono::prelude::*;
use thiserror::Error;

const IMAGE_DIRECTORY: &str = "./images";
const FILE_DIRECTORY: &str = "./files";

#[derive(Debug, Error)]
pub enum FileError {
    #[error("File io error `{0}`")]
    FileIoError(#[from] std::io::Error),
    #[error("File `{0}` doesnt exist")]
    FileDoesntExists(String),
    #[error("Convert of image to PNG failed with error `{0}`")]
    ConvertFailure(String),
    #[error("Directory `{0}` doesnt exist")]
    DirectoryDoesntExist(String),
}

pub fn read_file_to_vec(path: &String) -> Result<Vec<u8>, FileError> {
    let file_exists = file_exists(path)?;

    if !file_exists {
        return Err(FileError::FileDoesntExists(path.to_owned()).into());
    }

    let data = fs::read(path).map_err(|err| FileError::FileIoError(err))?;

    Ok(data)
}


pub fn save_and_convert_image(file: &str, content: &[u8]) -> Result<(), FileError> {
    let image_directory_exists = directory_exists(&IMAGE_DIRECTORY.to_string())?;

    if !image_directory_exists {
        return Err(FileError::DirectoryDoesntExist(IMAGE_DIRECTORY.to_string()).into());
    }

    let path = Path::new(file);
    let filename = path.file_name().unwrap().to_str().unwrap();
    let filepath = format!("{}/{}", IMAGE_DIRECTORY, filename);
    let mut file = File::create(filepath.clone()).map_err(|err| FileError::FileIoError(err))?;
    let current_timestamp = Local::now().timestamp();

    file.write_all(content).map_err(|err| FileError::FileIoError(err))?;

    let img = ImageReader::open(filepath.clone())
        .map_err(|err| FileError::FileIoError(err))?
        .decode().map_err(|err| FileError::ConvertFailure(err.to_string()))?;

    img.save(format!("{}/{}.png", IMAGE_DIRECTORY, current_timestamp)).map_err(|err| FileError::ConvertFailure(err.to_string()))?;
    fs::remove_file(filepath.clone())?;
    Ok(())
}

pub fn save_file(file: &str, content: &[u8]) -> Result<(), FileError> {
    let file_directory_exists = directory_exists(&FILE_DIRECTORY.to_string())?;

    if !file_directory_exists {
        return Err(FileError::DirectoryDoesntExist(FILE_DIRECTORY.to_string()).into());
    }

    let path = Path::new(file);
    let filename = path.file_name().unwrap().to_str().unwrap();
    let mut file = File::create(format!("{}/{}", FILE_DIRECTORY, filename)).map_err(|err| FileError::FileIoError(err))?;

    Ok(file.write_all(content).map_err(|err| FileError::ConvertFailure(err.to_string()))?)
}

fn file_exists(path: &String) -> Result<bool, FileError> {
    let metadata = fs::metadata(path).map_err(|err| FileError::FileIoError(err))?;

    Ok(metadata.is_file())
}

fn directory_exists(path: &String) -> Result<bool, FileError> {
    let metadata = fs::metadata(path).map_err(|_err| FileError::DirectoryDoesntExist(path.to_string()))?;

    Ok(metadata.is_dir())
}