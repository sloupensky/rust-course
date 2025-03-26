use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use image::ImageReader;
use chrono::prelude::*;
use thiserror::Error;

pub const IMAGE_DIRECTORY: &str = "./images";
pub const FILE_DIRECTORY: &str = "./files";

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

/// Reads file from path to vector
///
/// Returns FileError when failed
/// 
/// # Examples
/// ```
///  use file_utils::{read_file_to_vec}; 
///  std::env::set_current_dir("../../").expect("Failed to change directory");
///
///  let path = "example.txt".to_string();
///  let Ok(data) = read_file_to_vec(&path) else {
///     panic!("error reading file");
///  };
/// ```
pub fn read_file_to_vec(path: &String) -> Result<Vec<u8>, FileError> {
    let file_exists = file_exists(path)?;

    if !file_exists {
        return Err(FileError::FileDoesntExists(path.to_owned()).into());
    }

    let data = fs::read(path).map_err(|err| FileError::FileIoError(err))?;

    Ok(data)
}

/// Save and convert image
///
/// Gets image file from bin data, convert it into PNG format and saves it to IMAGE_DIRECTORY (./images/)
/// renamed to current timestamp
///
/// # Examples
/// ```
///  use file_utils::{save_and_convert_image,read_file_to_vec}; 
///  std::env::set_current_dir("../../").expect("Failed to change directory");
///
///  let file = "example.jpg".to_string();
///  let Ok(binary_data) = read_file_to_vec(&file) else {
///     panic!("error reading file");
///  };
///  let Ok(image) = save_and_convert_image("example.jpg", &binary_data) else {
///     panic!("error saving image");
///  };
/// ```
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

/// Save and convert image
///
/// Gets file from bin data and saves it to FILES_DIRECTORY (./files/)
/// renamed to current timestamp
///
/// # Examples
/// ```
///  use file_utils::{read_file_to_vec, save_file}; 
///  std::env::set_current_dir("../../").expect("Failed to change directory");
///
///  let file = "example.txt".to_string();
///  let Ok(binary_data) = read_file_to_vec(&file) else {
///     panic!("error reading file");
///  };
///  let Ok(result) = save_file("example.txt", &binary_data) else {
///     panic!("error saving file");
///  };
/// ```
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