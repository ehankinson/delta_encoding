// use std::collections::HashMap;
use rustc_hash::FxHashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};

const DNA_BASES: u32 = 5;

// pub fn read_book_content(filename: String) -> Result<HashMap<String, Vec<u32>>> {
//     let file = File::open(filename)?;
//     let reader = BufReader::new(file);

//     let mut index = 0;
//     let mut delta_encoding: HashMap<String, Vec<u32>> = HashMap::new();

//     for lines in reader.lines() {
//         let line = lines?;

//         for word in line.split_whitespace() {
//             let clean_word: String = word
//                 .chars()
//                 .filter(|c| c.is_alphanumeric())
//                 .flat_map(|c| c.to_lowercase())
//                 .collect();

//             if clean_word.is_empty() {
//                 continue;
//             }
//             delta_encoding
//                 .entry(clean_word)
//                 .or_default()
//                 .push(index as u32);

//             index += 1;
//         }
//     }
//     encode_deltas(&mut delta_encoding);
//     Ok(delta_encoding)
// }

pub fn read_dna_content(filename: String, k: u32) -> Result<FxHashMap<u32, Vec<u32>>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut map: FxHashMap<u32, Vec<u32>> = FxHashMap::default();

    let mut kmer = 0 as u32;
    let mut built = 0 as u32;
    let mut index = 0 as u32;

    let mut buf = vec![0u8; 16 * 1024 * 1024];

    let power_table = build_power_cache(DNA_BASES, k);
    let highest_power = power_table.last().unwrap();

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
            } else {
                let left_digit = kmer / highest_power;
                kmer = (kmer - left_digit * highest_power) * DNA_BASES+ digit;
            }

            map.entry(kmer)
                .or_insert_with(|| Vec::with_capacity(4))
                .push(index);
            index += 1;
        }
    }

    encode_deltas(&mut map);
    Ok(map)
}

fn encode_deltas(map: &mut FxHashMap<u32, Vec<u32>>) {
    for indices in map.values_mut() {
        for i in (1..indices.len()).rev() {
            indices[i] = indices[i] - indices[i - 1] as u32;
        }
    }
}

fn build_power_cache(base: u32, k: u32) -> Vec<u32> {
    let mut power_table = Vec::with_capacity(k as usize);
    power_table.push(1);
    for i in 1..k as usize {
        power_table.push(power_table[i - 1] * base);
    }

    power_table
}

fn get_digit(byte: u8) -> u32 {
    match byte {
        b'A' => 0,
        b'C' => 1,
        b'G' => 2,
        b'T' => 3,
        b'N' => 4,
        _ => 0,
    }
}
