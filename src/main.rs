use clap::{App, AppSettings, Arg};
use system_theme::*;

fn main() {
    let config = Config::from_file("/etc/system_theme_config.toml").expect("Failed to load config");
    let kde_current_theme = match list_current_kde_theme(&config.kdeglobals_path, &config.pattern) {
        Ok(theme) => theme,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    let matches = App::new("System Theme")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("list")
                .short('l')
                .long("list")
                .value_name("LIST")
                .help("List the current system theme")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("toggle")
                .short('t')
                .long("toggle")
                .value_name("TOGGLE")
                .help("Toggle the current system light/dark")
                .takes_value(false),
        )
        .get_matches();

    if matches.is_present("list") {
        println!("{}", kde_current_theme);
    }

    if matches.is_present("toggle") {
        println!("{}", kde_current_theme);
        if let Err(err) = toggle_alacritty_theme(&kde_current_theme, &config.alacritty_toml_path) {
            eprint!("Error toggling Alacritty theme: {}", err);
        }
        if kde_current_theme == config.dark_theme {
            if let Err(err) = toggle_kde_theme(&config.light_theme) {
                eprint!("Error toggling theme: {}", err);
            }
        } else if kde_current_theme == config.light_theme {
            if let Err(err) = toggle_kde_theme(&config.dark_theme) {
                eprint!("Error toggling theme: {}", err);
            }
        } else {
            println!("Current theme does not have logic to switch to")
        }
    }
}
