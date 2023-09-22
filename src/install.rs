use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

static SERVICE_NAME: &str = "com.danitt.monmon";

pub fn run(path_to_binary: &str) -> std::io::Result<()> {
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
              <string>~/Library/logs/{}.log</string>
              <key>StandardErrorPath</key>
              <string>~/Library/logs/{}.error.log</string>
          </dict>
          </plist>
      "#,
        SERVICE_NAME,
        path_to_binary,
        get_plist_name(),
        get_plist_name()
    );

    // Write plist to file
    let mut file = NamedTempFile::new()?;
    file.write_all(plist_content.as_bytes())?;
    let temp_file_path = file.path().to_str().unwrap();

    // Move plist file to the correct location
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

fn get_plist_name() -> String {
    format!("{}.plist", SERVICE_NAME)
}

pub fn get_plist_path() -> String {
    let home_dir = std::env::var("HOME").expect("Home directory not found");
    format!("{}/Library/LaunchAgents/{}", home_dir, get_plist_name())
}
