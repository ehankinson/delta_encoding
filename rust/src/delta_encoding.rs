use crate::constants::Posting;
use std::collections::HashMap;


pub fn delta_encoding(word_freq: HashMap<String, Vec<i32>>) -> Vec<Posting<i32>> {
    let mut postings: Vec<Posting<i32>> = Vec::new();

    for (word, freq) in word_freq.iter() {
        let mut payload: Vec<i32> = Vec::new();

        for &value in freq.iter().skip(1) {
            payload.push(value as i32);
        }

        postings.push(Posting {
            word: word.clone(),
            n: freq.len() as u32,
            base: freq[0] as u32,
            payload: payload,
            exceptions: exceptions,
        });
    }

    postings
}