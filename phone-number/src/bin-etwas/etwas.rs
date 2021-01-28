extern crate regex;
use regex::Regex;

fn main(){
    let user_number = "+1 (613)-995-0253";

    // Retain only numbers
    user_number.to_string().retain(|c| c.is_numeric());

    // Regex to match eleven digits
    let re = Regex::new(r"\d{10}|\b1+\d{10}").unwrap();
    re.captures(user_numer).unwrap_or(None).get(0).map_or(None, |m| Some(m.as_str()));
}