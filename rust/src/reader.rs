use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};

pub fn read_book_content(filename: String) -> Result<HashMap<String, Vec<u32>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut index = 0;
    let mut delta_encoding: HashMap<String, Vec<u32>> = HashMap::new();

    for lines in reader.lines() {
        let line = lines?;

        for word in line.split_whitespace() {
            let clean_word: String = word
                .chars()
                .filter(|c| c.is_alphanumeric())
                .flat_map(|c| c.to_lowercase())
                .collect();

            if clean_word.is_empty() {
                continue;
            }
            delta_encoding
                .entry(clean_word)
                .or_default()
                .push(index as u32);

            index += 1;
        }
    }
    encode_deltas(&mut delta_encoding);
    Ok(delta_encoding)
}

pub fn encode_deltas(map: &mut HashMap<String, Vec<u32>>) {
    for indices in map.values_mut() {
        for i in (1..indices.len()).rev() {
            indices[i] = indices[i] - indices[i - 1] as u32;
        }
    }
}

pub fn read_dna_content(filename: String, k: usize) -> Result<HashMap<String, Vec<u32>>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut delta_encoding: HashMap<String, Vec<u32>> = HashMap::new();

    let mut index = 0 as u32;
    let mut buf = vec![0u8; 1024 * 1024];

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }

        process_chunk(&mut delta_encoding, &buf, k, &mut index);
    }

    encode_deltas(&mut delta_encoding);
    Ok(delta_encoding)
}

fn process_chunk(map: &mut HashMap<String, Vec<u32>>, buf: &[u8], k: usize, index: &mut u32) {
    if buf.len() < k {
        return;
    }

    for i in 0..buf.len() - k {
        let slice = &buf[i..i + k];
        let kmer: String = slice.iter().map(|&c| c as char).collect();
        map.entry(kmer).or_default().push(*index);
        *index += 1;
    }
}
