use crate::constants::Posting;
use std::collections::HashMap;

pub fn varint_encode (word_freq: &HashMap<String, Vec<u32>>) -> Vec<Posting>{
    let mut postings: Vec<Posting> = Vec::new();

    for (word, freq) in word_freq.iter(){
        let mut payload: Vec<u8> = Vec::new();
         for &value in freq.iter().skip(1) {
            let mut temp = value;
            while temp >= 0x80 {
                payload.push(((temp & 0x7F) | 0x80) as u8);
                temp >>= 7;
            }
            payload.push(temp as u8);
        }

        postings.push(Posting {
            word: word.clone(),
            n: freq.len() as u32,
            base: freq[0] as u32,
            payload: payload,
            exceptions: None,
        });
    }
    postings
}