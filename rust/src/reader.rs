use std::fs;

pub fn read_content(filename: String) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    let words: Vec<String> = contents
        .split_whitespace()
        .map(|s| s.to_string().to_lowercase())
        .collect();

    words
}
