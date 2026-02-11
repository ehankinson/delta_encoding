use std::collections::HashMap;

pub fn byte_pack_encode(word_freq: HashMap<String, Vec<i32>>) {
    let mut offset: u32 = 0;
    for (word, freq) in word_freq.iter() {
        let base = freq.first().unwrap();
        

    }
}