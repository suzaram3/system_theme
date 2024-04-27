use system_theme::{self, list_current_theme};

fn main() {
    let home_dir = std::env::var("HOME").expect("HOME var not set.");
    let kdeglobals_path = format!("{}/.config/kdeglobals", home_dir);
    let pattern = "lookandfeelpackage";

    let contents = list_current_theme(&kdeglobals_path, &pattern);

    print!("Contents: {:?}", contents);
}
