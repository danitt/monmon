use super::env;
use std::fs::create_dir_all;
use std::io::Write;
use std::process::Command;
use std::str;
use tempfile::NamedTempFile;

static SERVICE_NAME: &str = "com.danitt.monmon";

pub fn run() -> std::io::Result<()> {
    let blacklisted_displays_str = env::get_blacklisted_displays().join(",");
    let output_log = format!("{}/Library/logs/{}.log", get_home_dir(), get_plist_name());
    let error_log = format!(
        "{}/Library/logs/{}.error.log",
        get_home_dir(),
        get_plist_name()
    );
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
          <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
          <plist version="1.0">
          <dict>
              <key>Label</key>
              <string>{}</string>
              <key>ProgramArguments</key>
              <array>
                  <string>{}</string>
                  <string>watch</string>
              </array>
              <key>RunAtLoad</key>
              <true/>
              <key>KeepAlive</key>
              <true/>
              <key>StandardOutPath</key>
              <string>{}</string>
              <key>StandardErrorPath</key>
              <string>{}</string>
              <key>EnvironmentVariables</key>
              <dict>
                  <key>BLACKLIST_DISPLAYS</key>
                  <string>{}</string>
              </dict>
          </dict>
          </plist>
      "#,
        SERVICE_NAME,
        get_path_to_binary(),
        output_log,
        error_log,
        blacklisted_displays_str
    );

    // Ensure the LaunchAgents directory exists
    create_dir_all(&get_plist_dir()).expect("Failed to create LaunchAgents directory");

    // Write plist to file
    let mut file = NamedTempFile::new()?;
    file.write_all(plist_content.as_bytes())?;
    let temp_file_path = file.path().to_str().unwrap();

    // Move plist file to launch agents directory
    Command::new("mv")
        .args(&[temp_file_path, &get_plist_path()])
        .output()
        .expect("Failed to move plist file.");

    // Load the service using launchctl
    Command::new("launchctl")
        .args(&["load", &get_plist_path()])
        .output()
        .expect("Failed to load service.");

    println!("Service has been installed and started.");
    Ok(())
}

fn get_path_to_binary() -> String {
    let output = Command::new("which")
        .arg("monmon")
        .output()
        .expect("Failed to execute command");

    str::from_utf8(&output.stdout)
        .expect("Could not convert to UTF-8")
        .trim() // Remove trailing newline
        .to_string()
}

fn get_home_dir() -> String {
    std::env::var("HOME").expect("Home directory not found")
}

fn get_plist_dir() -> String {
    format!("{}/Library/LaunchAgents", get_home_dir())
}

fn get_plist_name() -> String {
    format!("{}.plist", SERVICE_NAME)
}

pub fn get_plist_path() -> String {
    format!("{}/{}", get_plist_dir(), get_plist_name())
}
