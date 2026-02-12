use crate::constants::BytePosting;
use std::collections::HashMap;

pub fn byte_pack_encode(word_freq: HashMap<String, Vec<i32>>) -> Vec<BytePosting> {
    let mut postings: Vec<BytePosting> = Vec::new();

    for (word, freq) in word_freq.iter() {
        let mut payload: Vec<u8> = Vec::new();
        let mut exceptions: Vec<u16> = Vec::new();

        for &value in freq.iter().skip(1) {
            if value <= 255 {
                payload.push(value as u8);
            } else {
                payload.push(0);
                exceptions.push(value as u16);
            }
        }

        postings.push(BytePosting {
            word: word.clone(),
            n: freq.len() as u32,
            base: freq[0] as u32,
            payload: payload,
            exceptions: exceptions,
        });
    }

    postings
}
