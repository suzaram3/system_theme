use clap::{App, AppSettings, Arg};
use system_theme::*;

fn main() {
    let config = Config::from_file("config.toml").expect("Failed to load config");

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
        let contents = list_current_theme(&config.kdeglobals_path, &config.pattern);
        match contents {
            Ok(theme) => println!("Current system theme: {}", theme),
            Err(err) => eprint!("Error: {}", err),
        }
    }
}
