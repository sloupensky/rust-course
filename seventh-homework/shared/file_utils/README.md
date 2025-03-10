# FILE UTILS
This is helper library used with file operations

* `read_file_to_vec(path: &String -> Result<Vec<u8>, Box<dyn Error>>)` will read file to vector
* `save_and_convert_image(file: &str, content: &[u8]) -> Result<(), Box<dyn Error>>` will read an image, then it will convert it and save to `./images/` folder with current timestamp
* `save_file(file: &str, content: &[u8]) -> Result<(), Box<dyn Error>>` will just save the file into `./files/` folder with same name