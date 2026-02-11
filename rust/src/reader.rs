use std::fs;

pub fn read_content(filename: String) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    let words: Vec<String> = contents
        .split_whitespace()
        .map(|s| {
            s.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
        })
        .filter(|s| !s.is_empty())
        .collect();

    words
}
