extern crate regex;
use regex::Regex;
use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let re = Regex::new(r"\w+(?:[']?[[:alnum:]])*").unwrap();
    let mut hmap: HashMap<String, u32> = HashMap::new();
    re.captures_iter(words)
        .map(|w| w[0].to_lowercase())
        .for_each(|w| {
            *hmap.entry(w).or_insert(0) += 1;
        }
    );
    hmap
}