mod const;

use std::collections::HashMap;

pub fn byte_pack_encode(word_freq: HashMap<String, Vec<i32>>) -> Vec<Posting> {
    let mut offset: u32 = 0;
    let mut postings: Vec<Posting> = Vec::new();

    for (word, freq) in word_freq.iter() {
        let mut payload: Vec<u8> = Vec::new();
        let mut exceptions: Vec<u16> = Vec::new();

        for &value in freq.iter().skip(1) {
            if value <= 255 {
                payload.push(*value as u8);
            }
            else {
                payload.push(0);
                exceptions.push(*value as u16);
            }
        }

        postings.push(
            Posting {
                n: freq.len() as u32,
                base: freq.first().unwrap() as u32,
                payload_length: payload.len() as u32,
                payload: payload,
                exception_length: exceptions.len() as u32,
                exceptions: exceptions
            }
        )
    }

    postings
}