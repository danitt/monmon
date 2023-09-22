use std::fs;
use std::process::Command;

use super::install::get_plist_path;

pub fn run() -> std::io::Result<()> {
    // Unload the service using launchctl
    Command::new("launchctl")
        .args(&["unload", &get_plist_path()])
        .output()
        .expect("Failed to unload service.");

    // Remove plist file
    fs::remove_file(&get_plist_path()).expect("Failed to remove plist file.");

    println!("Service has been uninstalled.");
    Ok(())
}
