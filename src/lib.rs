use std::fs::File;
use std::io::{self, Read};

pub fn list_current_theme(file_path: &str, pattern: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut current_theme = String::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(pattern) {
            current_theme = line.split('=').nth(1).unwrap().trim().to_string();
            break;
        }
    }
    Ok(current_theme)
}
