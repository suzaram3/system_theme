use std::fs::File;
use std::io::{self, Read};
use std::process::Command;
use toml::Value;

#[derive(Debug)]
pub struct Config {
    pub kdeglobals_path: String,
    pub pattern: String,
    pub dark_theme: String,
    pub light_theme: String,
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
        let dark_theme = toml_value["kde_themes"]["dark"]
            .as_str()
            .ok_or("dark theme not found in config file")?
            .to_string()
            .to_lowercase();
        let light_theme = toml_value["kde_themes"]["light"]
            .as_str()
            .ok_or("light theme not found in config file")?
            .to_string()
            .to_lowercase();

        Ok(Config {
            kdeglobals_path,
            pattern,
            dark_theme,
            light_theme,
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

pub fn toggle_current_theme(theme: &str) -> Result<(), io::Error> {
    let output = Command::new("lookandfeeltool")
        .arg("-a")
        .arg(theme)
        .output()
        .expect("Faield to execute command");

    if output.status.success() {
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to execute command: {}", error_message),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_current_theme_light() {
        let config = Config::from_file("config.toml").expect("Failed to load config");

        let result = list_current_theme(&config.kdeglobals_path, &config.pattern).unwrap();

        assert_eq!(result, "org.manjaro.breath-light.desktop".to_string());
    }

    #[test]
    fn test_list_current_theme_dark() {
        let config = Config::from_file("config.toml").expect("Failed to load config");

        let result = list_current_theme(&config.kdeglobals_path, &config.pattern).unwrap();

        assert_eq!(result, "org.manjaro.breath-dark.desktop".to_string());
    }

    #[test]
    fn test_toggle_theme_dark() {
        let theme = "org.manjaro.breath-dark.desktop";
        if let Err(err) = toggle_current_theme(theme) {
            eprint!("Error toggling theme: {}", err);
        }

        #[test]
        fn test_toggle_theme_light() {
            let theme = "org.manjaro.breath-light.desktop";
            if let Err(err) = toggle_current_theme(theme) {
                eprint!("Error toggling theme: {}", err);
            }
        }
    }
}
