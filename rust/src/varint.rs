use crate::constants::Posting;
use std::collections::HashMap;

pub fn varint_encode (word_freq: &HashMap<String, Vec<u32>>) -> Vec<Posting>{
    let mut postings: Vec<Posting> = Vec::new();

    for (word, freq) in word_freq.iter(){
        let mut payload: Vec<u8> = Vec::new();
         for &value in freq.iter().skip(1) {
            let mut iterations = (value.ilog(128) as f32).ceil() as u32;
            let mut temp = value;
            while iterations > 1 {
                let seven_bits = ((temp & 0x7F) | 0x80) as u8;
                temp = temp >> 7;
                payload.push(seven_bits);
                iterations -= 1;
            }

            let x: u8 = temp as u8;
            payload.push(((0x7F & x)) as u8)

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