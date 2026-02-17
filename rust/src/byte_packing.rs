use crate::constants::Posting;
use std::collections::HashMap;

pub fn byte_pack_encode(word_freq: &HashMap<String, Vec<u32>>) -> Vec<Posting> {
    let mut postings: Vec<Posting> = Vec::with_capacity(word_freq.len());

    for (word, freq) in word_freq.iter() {
        let tail_len = freq.len().saturating_sub(1);
        let mut payload: Vec<u8> = Vec::with_capacity(tail_len);
        let mut exceptions: Option<Vec<u8>> = None;

        for &value in &freq[1..] {
            if value <= 255 {
                payload.push(value as u8);
            } else {
                payload.push(0);
                let ex = exceptions.get_or_insert_with(|| Vec::with_capacity(8));
                ex.push((value & 0xFF) as u8);
                ex.push(((value >> 8) & 0xFF) as u8);
            }
        }

        postings.push(Posting {
            word: word.clone(),
            n: freq.len() as u32,
            base: freq[0] as u32,
            payload: payload,
            exceptions,
        });
    }

    postings
}
