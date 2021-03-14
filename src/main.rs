extern crate regex;
extern crate reqwest;
extern crate serde_json;
extern crate termion;

use std::collections::HashMap;
use std::error::Error;

use regex::Regex;
use termion::color;

fn main() {
    one().unwrap_or_else(|err| {
        eprintln!("There was an error in function one: {}", err);
    });

    two().unwrap_or_else(|err| {
        eprintln!("There was an error inside function two: {}", err);
    });
}

fn one() -> Result<(), Box<dyn Error>> {
    let res = reqwest::blocking::get("https://tmi.twitch.tv/group/user/moonmoon/chatters")?.text().unwrap();
    let json_d: serde_json::Value = serde_json::from_str(&res).expect("Error");
    println!("Chatter count for moonmoon: {}{}{}", color::Fg(color::Green), json_d["chatter_count"], color::Fg(color::Reset));

    let re = Regex::new(r"\bnightbot\b").unwrap();
    match re.captures(&res) {
        Some(cap) => println!("Found {}{}{} in the chatter list\n", color::Fg(color::LightCyan), &cap[0], color::Fg(color::Reset)),
        None => println!("Not found")
    }
    Ok(())
}

fn two() -> Result<(), Box<dyn Error>> {
    let res = reqwest::blocking::get("https://httpbin.org/ip")?.json::<HashMap<String, String>>()?;
    println!("Your IP: {}{}{}", color::Fg(color::Magenta), res["origin"], color::Fg(color::Reset));
    Ok(())
}
