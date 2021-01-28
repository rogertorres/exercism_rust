extern crate regex;
use regex::Regex;

pub fn number(user_number: &str) -> Option<String> {
    // Retain only numbers
    let mut n = String::from(user_number);
    n.retain(|c| c.is_numeric());

    // Regex to match eleven digits
    let re = Regex::new(r"^(\d{10}|\b1\d{10})$").unwrap();
    if re.is_match(&n) { 
        Some(n) 
    } else { 
        None
    }

    // return re.captures(user_number).unwrap_or_else(|| regex::Captures::None).get(0).map_or(None, |m| Some(m.as_str().to_string()));
}





// unimplemented!(
//     "Given the number entered by user '{}', 
//     convert it into SMS-friendly format. If the entered number is not a valid NANP number, 
//     return None.",
//     user_number
// );
