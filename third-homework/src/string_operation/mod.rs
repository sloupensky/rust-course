mod csv;
use slug::slugify;
use std::error::Error;
use std::io::{Lines, StdinLock};

pub fn modify_string(lines: Lines<StdinLock<'static>>, arguments: Vec<String>) -> Result<String, Box<dyn Error>> {
    let string = self::concatenate_lines(lines)?;

    if string.len() == 1 {
        return Err("String cannot be empty".into());
    }
    if arguments.len() > 2 {
        return Err(format!("There is only one allowed argument, you entered {}", arguments.len() - 1).into());
    } else if arguments.len() < 2 || arguments[1].is_empty() {
        return Err("Argument cannot be empty".into());
    }

    let modification_type = arguments[1].as_str();

    match modification_type {
        "lowercase" => string_to_lowercase(string),
        "uppercase" => string_to_uppercase(string),
        "no-spaces" => string_without_whitespaces(string),
        "slugify" => slugified_string(string),
        "csv" => csv(string),
        _ => Err(format!("Unknown modification parameter {}", modification_type).into()),
    }
}

fn concatenate_lines(lines: Lines<StdinLock<'static>>) -> Result<String, Box<dyn Error>> {
    let string_lines:Vec<String> = lines.map(|line| line.unwrap()).collect();

    Ok(string_lines.join("\n"))
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
