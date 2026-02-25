use crate::constants::DNA_BASES;
use crate::util::decode_dna_kmer;
use crate::util::get_digit;
use rustc_hash::FxHashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};

pub fn read_book_content(filename: String) -> Result<(FxHashMap<u32, Vec<u32>>, Vec<Vec<u8>>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut term_id = 0;
    let mut index = 0 as u32;
    let mut temp: FxHashMap<u32, (u32, Vec<u32>)> = FxHashMap::default();
    let mut term_to_id: FxHashMap<String, u32> = FxHashMap::default();
    let mut terms = Vec::new();

    for line in reader.lines() {
        let line = line?;

        for word in line.split_whitespace() {
            let clean_word: String = word
                .chars()
                .filter(|c| c.is_alphanumeric())
                .flat_map(|c| c.to_lowercase())
                .collect();

            if clean_word.is_empty() {
                continue;
            }

            if !term_to_id.contains_key(&clean_word) {
                let term_bytes = Vec::from(clean_word.as_bytes());
                term_to_id.insert(clean_word, term_id);
                terms.push(term_bytes);
                temp.insert(term_id, (index, vec![index]));
                term_id += 1;
            } else {
                let current_id = term_to_id.get(&clean_word).unwrap();
                if let Some((last_index, indices)) = temp.get_mut(current_id) {
                    indices.push(index - *last_index);
                    *last_index = index;
                }
            }

            index += 1;
        }
    }

    let final_map: FxHashMap<u32, Vec<u32>> = temp
        .into_iter()
        .map(|(term_id, (_last, indices))| (term_id, indices))
        .collect();

    Ok((final_map, terms))
}

pub fn read_dna_content(
    filename: String,
    k: u32,
) -> Result<(FxHashMap<u32, Vec<u32>>, Vec<Vec<u8>>)> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut term_to_id: FxHashMap<u32, u32> = FxHashMap::default();
    let mut temp: FxHashMap<u32, (u32, Vec<u32>)> = FxHashMap::default();
    let mut terms: Vec<Vec<u8>> = Vec::new();

    if k == 0 {
        return Ok((FxHashMap::default(), terms));
    }

    let mut kmer = 0u32;
    let mut built = 0u32;
    let mut index = 0u32;
    let highest_power = DNA_BASES.pow(k - 1);

    let mut buf = vec![0u8; 16 * 1024 * 1024];

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }

        for &byte in &buf[..n] {
            let digit = get_digit(byte);

            if built < k {
                kmer = kmer * DNA_BASES + digit;
                built += 1;
                if built < k {
                    index += 1;
                    continue;
                }
            } else {
                let left_digit = kmer / highest_power;
                kmer = (kmer - left_digit * highest_power) * DNA_BASES + digit;
            }

            let token_index = index + 1 - k;
            if let Some(term_id) = term_to_id.get(&kmer).copied() {
                if let Some((last_index, indices)) = temp.get_mut(&term_id) {
                    indices.push(token_index - *last_index);
                    *last_index = token_index;
                }
            } else {
                let term_id = terms.len() as u32;
                term_to_id.insert(kmer, term_id);
                terms.push(decode_dna_kmer(kmer, k));
                temp.insert(term_id, (token_index, vec![token_index]));
            }

            index += 1;
        }
    }

    let final_map: FxHashMap<u32, Vec<u32>> = temp
        .into_iter()
        .map(|(term_id, (_last, indices))| (term_id, indices))
        .collect();

    Ok((final_map, terms))
}
