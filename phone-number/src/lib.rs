extern crate regex;
use regex::Regex;

pub fn number(user_number: &str) -> Option<String> {
    // Retain only numbers
    let mut n = String::from(user_number);
    n.retain(|c| c.is_numeric());

    // Regex to match NANP number
    let re = Regex::new(r"\b1?([2-9]\d{2}[2-9]\d{6})\b").unwrap();
    match re.captures(&n) {
        Some(x) => Some(x[1].to_string()),
        None => None,
    }
}