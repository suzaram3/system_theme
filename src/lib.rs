use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read};
use std::process::Command;
use toml::{map::Map, Value};

#[derive(Debug)]
pub struct Config {
    pub kdeglobals_path: String,
    pub pattern: String,
    pub dark_theme: String,
    pub light_theme: String,
    pub alacritty_toml_path: String,
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
        let alacritty_toml_path = toml_value["alacritty"]["config_path"]
            .as_str()
            .ok_or("alacritty file path not found in config file")?
            .to_string()
            .to_lowercase();

        Ok(Config {
            kdeglobals_path,
            pattern,
            dark_theme,
            light_theme,
            alacritty_toml_path,
        })
    }
}

pub fn list_current_kde_theme(file_path: &str, pattern: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut kde_current_theme = String::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(pattern) {
            kde_current_theme = line.split('=').nth(1).unwrap().trim().to_string();
            break;
        }
    }
    Ok(kde_current_theme)
}

pub fn list_current_alacritty_theme(file_path: &str, pattern: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut alacritty_current_theme = String::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(pattern) {
            alacritty_current_theme = line.split('=').nth(1).unwrap().trim().to_string();
            break;
        }
    }
    Ok(alacritty_current_theme)
}

pub fn toggle_kde_theme(theme: &str) -> Result<(), io::Error> {
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

pub fn toggle_alacritty_theme(current_theme: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let mut config: Map<String, Value> = toml::from_str(&contents)?;

    if let Some(import) = config.get_mut("import") {
        if let Value::Array(ref mut imports) = *import {
            if let Some(first_import) = imports.get_mut(0) {
                if let Value::String(import_value) = first_import {
                    if current_theme == "org.manjaro.breath-dark.desktop" {
                        *import_value =
                            "~/.config/alacritty/alacritty-theme/themes/solarized_light.toml"
                                .to_string();
                    } else if current_theme == "org.manjaro.breath-light.desktop" {
                        *import_value =
                            "~/.config/alacritty/alacritty-theme/themes/solarized_dark.toml"
                                .to_string();
                    }
                }
            }
        }
    }
    let updated_contents = toml::to_string_pretty(&config)?;
    fs::write(file_path, updated_contents)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_current_theme_light() {
        let config = Config::from_file("config.toml").expect("Failed to load config");

        let result = list_current_kde_theme(&config.kdeglobals_path, &config.pattern).unwrap();

        assert_eq!(result, "org.manjaro.breath-light.desktop".to_string());
    }
}
