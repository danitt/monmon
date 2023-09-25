use dotenv::dotenv;
use std::env;

pub fn get_blacklisted_displays() -> Vec<String> {
    dotenv().ok();
    let blacklist_displays_str =
        env::var("BLACKLIST_DISPLAYS").expect("BLACKLIST_DISPLAYS must be set");
    blacklist_displays_str
        .split(",")
        .map(|s| s.to_string())
        .collect()
}
