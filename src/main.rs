extern crate dotenv;
use clap::{Parser, Subcommand};
mod env;
mod install;
mod uninstall;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use std::time::Duration;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(global = true, short = 'b', long)]
    blacklist: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Watch,
    Install,
    Uninstall,
}

fn main() {
    let args = Cli::parse();

    // Set BLACKLIST_DISPLAYS environment variable if provided
    if args.blacklist.is_some() {
        let blacklist: Vec<String> = args
            .blacklist
            .unwrap()
            .split(",")
            .map(|s| s.to_string())
            .collect();
        env::set_blacklisted_displays(blacklist);
    }

    match args.command {
        Some(Commands::Watch) => {
            watch().unwrap();
        }
        Some(Commands::Install) => {
            install::run().unwrap();
        }
        Some(Commands::Uninstall) => {
            uninstall::run().unwrap();
        }
        None => move_windows_to_primary_display(),
    }
}

/*
 * Watches for changes to the display configuration and moves windows to the primary display
 */
fn watch() -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let config = Config::default().with_poll_interval(Duration::from_secs(2));
    let mut watcher = RecommendedWatcher::new(tx, config)?;
    watcher.watch(
        Path::new("/Library/Preferences/com.apple.windowserver.displays.plist"),
        RecursiveMode::NonRecursive,
    )?;

    println!("Watching for monitor configuration changes...");
    for res in rx {
        match res {
            Ok(_) => {
                if !is_blacklisted_display_connected() {
                    println!("No blacklisted display found.");
                    () // No blacklisted display found, do nothing
                } else {
                    println!("Blacklisted monitor detected, moving windows to primary display.");
                    move_windows_to_primary_display();
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    Ok(())
}

fn is_blacklisted_display_connected() -> bool {
    let blacklist_displays = env::get_blacklisted_displays();

    let mut is_blacklisted_display_connected = false;
    get_connected_displays().iter().for_each(|display| {
        let display_lower = display.to_lowercase();
        if blacklist_displays
            .iter()
            .any(|d| d.to_lowercase() == display_lower)
        {
            is_blacklisted_display_connected = true;
        }
    });

    is_blacklisted_display_connected
}

fn move_windows_to_primary_display() {
    let applescript = r#"
      on moveAppWindows()
        tell application "System Events"
          set _apps to name of every application process whose visible is true
          repeat with _app in _apps
            try
              set _windows to every window of application process _app
              repeat with _win in _windows
                set position of _win to {0, 0}
              end repeat
            on error errMsg
              log "Error with app " & _app & ": " & errMsg
            end try
          end repeat
        end tell
      end moveAppWindows

      moveAppWindows()
    "#;

    Command::new("osascript")
        .arg("-e")
        .arg(&applescript)
        .output()
        .expect("Failed to execute AppleScript");
}

fn get_connected_displays() -> Vec<String> {
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute system_profiler command");

    let output_str = str::from_utf8(&output.stdout).expect("Could not convert to string");
    parse_connected_displays(output_str)
}

/**
 * Parses the output of `system_profiler SPDisplaysDataType` and returns a list of connected displays
 * Note: This logic makes a lot of assumptions about the output of `system_profiler SPDisplaysDataType`
 */
fn parse_connected_displays(output_str: &str) -> Vec<String> {
    let mut displays: Vec<String> = vec![];

    // Hacky way to capture the indentation level for the name of each connected display
    let mut display_indentation: Option<usize> = None;

    for (_, line) in output_str.lines().enumerate() {
        match display_indentation {
            None => {
                // We haven't found the "Displays:" line yet
                if line.trim() == "Displays:" {
                    display_indentation = Some(count_leading_whitespace(line) + 2);
                }
            }
            Some(indentation) => {
                if count_leading_whitespace(line) != indentation {
                    continue;
                }
                let mut display = line.trim();
                if display.ends_with(":") {
                    display = &display[..display.len() - 1];
                }
                displays.push(display.to_string());
            }
        }
    }

    displays
}

fn count_leading_whitespace(s: &str) -> usize {
    s.chars().take_while(|c| c.is_whitespace()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_connected_displays() {
        let output_str = r#"
          Graphics/Displays:

              Apple M2 Pro:

                Chipset Model: Apple M2 Pro
                Type: GPU
                Bus: Built-In
                Total Number of Cores: 16
                Vendor: Apple (0x106b)
                Metal Support: Metal 3
                Displays:
                  Color LCD:
                    Display Type: Built-in Liquid Retina XDR Display
                    Resolution: 3024 x 1964 Retina
                    Main Display: Yes
                    Mirror: Off
                    Online: Yes
                    Automatically Adjust Brightness: Yes
                    Connection Type: Internal
                  SAMSUNG:
                    Resolution: 3840 x 2160 (2160p/4K UHD 1 - Ultra High Definition)
                    UI Looks like: 1920 x 1080 @ 30.00Hz
                    Mirror: Off
                    Online: Yes
                    Rotation: Supported
                    Television: Yes
        "#;

        let result = parse_connected_displays(output_str);
        assert_eq!(result, vec!["Color LCD", "SAMSUNG"]);
    }

    #[test]
    fn test_count_leading_whitespace() {
        assert_eq!(count_leading_whitespace("  foo"), 2);
        assert_eq!(count_leading_whitespace("foo"), 0);
        assert_eq!(count_leading_whitespace("      foo"), 6);
    }
}
