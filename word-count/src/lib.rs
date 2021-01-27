extern crate regex;
use regex::Regex;
use std::collections::HashMap;

// use std::collections::BTreeMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let re = Regex::new(r"\w+(?:[']?[[:alnum:]])*").unwrap();
    let mut hmap: HashMap<String, u32> = HashMap::new();
    re.captures_iter(words)
        // .split(|c| [' ', ',', '\n', '\t'].contains(&c) )
        .map(|w| w[0].to_lowercase())
        .filter(|w| !w.trim().is_empty())
        .for_each(|w| {
            println!("{}",w);
            let counter = hmap.entry(w).or_insert(0);
            *counter += 1;
        }
    );

    hmap
}


/// Count occurrences of words.
pub fn _attempt_without_regex(words: &str) -> HashMap<String, u32> {
    let mut hmap: HashMap<String, u32> = HashMap::new();
    words
        .split(|c| [' ', ',', '\n', '\t'].contains(&c) )
        .map(|w| w.matches(char::is_alphanumeric).collect::<String>().to_lowercase())
        .filter(|w| !w.trim().is_empty())
        .for_each(|w| {
            println!("{}",w);
            let counter = hmap.entry(w).or_insert(0);
            *counter += 1;
        }
    );

    hmap
}

