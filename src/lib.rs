use std::fs::File;
use std::io::{self, Read};
use toml::Value;

#[derive(Debug)]
pub struct Config {
    pub kdeglobals_path: String,
    pub pattern: String,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str)?;

        let toml_value: Value = toml::from_str(&toml_str)?;
        let kdeglobals_path = toml_value["kde_config"]["kdeglobals_path"]
            .as_str()
            .ok_or("kdegloblas_path not found in config file")?
            .to_string();
        let pattern = toml_value["kde_config"]["pattern"]
            .as_str()
            .ok_or("pattern not found in config file")?
            .to_string()
            .to_lowercase();

        Ok(Config {
            kdeglobals_path,
            pattern,
        })
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_current_theme() {
        let temp_file_path = "test_file.txt";
        let temp_file_content = "Some line\nLookAndFeelPackage=ThemeName\nAnother line";
        std::fs::write(temp_file_path, temp_file_content).expect("Failed to write temp file");

        let result = list_current_theme(temp_file_path, "lookandfeelpackage").unwrap();

        assert_eq!(result, "ThemeName".to_string());

        std::fs::remove_file(temp_file_path).expect("Failed to remove temp file");
    }

    #[test]
    fn test_list_current_theme_light() {
        let temp_file_path = "/home/rinzler/.config/kdeglobals";

        let result = list_current_theme(temp_file_path, "lookandfeelpackage").unwrap();

        assert_eq!(result, "org.manjaro.breath-light.desktop".to_string());
    }
}
