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

/*
Sets the BLACKLIST_DISPLAYS environment variable,
overwriting if it already exists.
 */
pub fn set_blacklisted_displays(blacklist: Vec<String>) -> () {
    let blacklist_str = blacklist.join(",");
    env::set_var("BLACKLIST_DISPLAYS", blacklist_str);
}
