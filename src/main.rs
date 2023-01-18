mod observations;

use std::env::var;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    observations::get_observations().await;
    // observations::send_temp();
    Ok(())
}

// This function has been left in main.rs as this will also be used by a future forecasts.rs module
fn get_config_path() -> String {
    // Your config.toml should be in $HOME/.config/rusqttbom/config.toml
    // First, we need to identify the home drive
    // Then we can add on the remainder of the file path
    let home_dir = var("HOME");
    // We need to clean the start and end of this string
    println!("short path {:?}", home_dir);
    // We need to convert the type to String
    let hd_string = format!("{:?}", home_dir);
    let start = 5; // This is to remove formatting from start of file path
    let length = hd_string.len();
    let end = length - 2; // This is to remove same from end
    let clean_dir = &hd_string[start..end];
    // Now that we have a clean string we can amend the rest
    let config_path = format!("/{}/.config/rusqttbom/config.toml", clean_dir);
    config_path
}
