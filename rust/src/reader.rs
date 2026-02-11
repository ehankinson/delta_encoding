use std::fs;
use std::collections::HashMap;

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

//we make this function a life time. Meaning, we make a promise that the values of str wont be called after the word array
//dissapears. Instead of having a hashmap with more strings, we use their reference (pointers)
pub fn generate_freq(word_array: Vec<String>) -> HashMap< String, Vec<i32>>{
    let mut delta_encoding: HashMap<String, Vec<i32>> = HashMap::new();
    //Enumerate is created from an iter. The reason why this is necessary because you need to explicitly define what
    //you want to do. For example, a regular iter is a read only reference, iter_mut is a mutable reference
    for(i, string) in word_array.into_iter().enumerate() {
        delta_encoding
            .entry(string)
            .and_modify(|vec|{
                let last_pos = *vec.last().unwrap();
                vec.push(i as i32 - last_pos);
            }) 
            //makes the default value
            .or_default()
            .push(i as i32);
    } 
    delta_encoding
}
