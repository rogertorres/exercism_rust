use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut hmap: HashMap<String, u32> = HashMap::new();
    words
        .split(|c: char| [' ', ',','\n','\t'].contains(&c) )
        .map(|w| w)
        .filter(|w| !w.trim().is_empty()) 
        .for_each(|w| {
            let counter = hmap.entry(w.to_string()).or_insert(0);
            *counter += 1;
        }
    );

    hmap
}
