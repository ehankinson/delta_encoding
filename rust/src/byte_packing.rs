use crate::constants::Posting;
use std::collections::HashMap;

pub fn byte_pack_encode(word_freq: &HashMap<String, Vec<u32>>) -> Vec<Posting> {
    let mut postings: Vec<Posting> = Vec::new();

    for (word, freq) in word_freq.iter() {
        let mut payload: Vec<u8> = Vec::new();
        let mut exceptions: Vec<u8> = Vec::new();

        for &value in freq.iter().skip(1) {
            if value <= 255 {
                payload.push(value as u8);
            } else {
                payload.push(0);
                exceptions.push((value & 0xFF) as u8);
                exceptions.push(((value >> 8) & 0xFF) as u8);
            }
        }

        postings.push(Posting {
            word: word.clone(),
            n: freq.len() as u32,
            base: freq[0] as u32,
            payload: payload,
            exceptions: Some(exceptions),
        });
    }

    postings
}
