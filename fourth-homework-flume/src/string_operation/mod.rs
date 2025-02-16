mod csv;
use slug::slugify;
use std::error::Error;
use std::str::FromStr;

enum ModificationType {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    Csv,
}

impl FromStr for ModificationType {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "lowercase" => Ok(ModificationType::Lowercase),
            "uppercase" => Ok(ModificationType::Uppercase),
            "no-spaces" => Ok(ModificationType::NoSpaces),
            "slugify" => Ok(ModificationType::Slugify),
            "csv" => Ok(ModificationType::Csv),
            _ => Err(format!("Unknown modification type {}", s).into()),
        }
    }
}

pub fn modify_string(string: String, modification_type: &String) -> Result<String, Box<dyn Error>> {
    match ModificationType::from_str(modification_type) {
        Ok(ModificationType::Lowercase) => string_to_lowercase(string),
        Ok(ModificationType::Uppercase) => string_to_uppercase(string),
        Ok(ModificationType::NoSpaces) => string_without_whitespaces(string),
        Ok(ModificationType::Slugify) => slugified_string(string),
        Ok(ModificationType::Csv) => csv(string),
        Err(error) => Err(error.into()),
    }
}

fn string_to_lowercase(string: String) -> Result<String, Box<dyn Error>> {
    Ok(string.to_lowercase())
}

fn string_to_uppercase(string: String) -> Result<String, Box<dyn Error>> {
    Ok(string.to_uppercase())
}

fn string_without_whitespaces(string: String) -> Result<String, Box<dyn Error>> {
    Ok(string.replace(" ", ""))
}

fn slugified_string(string: String) -> Result<String, Box<dyn Error>> {
    Ok(slugify(string))
}

fn csv(string: String) -> Result<String, Box<dyn Error>> {
    csv::tabify_string_from_csv(string, String::from(";"))
}
