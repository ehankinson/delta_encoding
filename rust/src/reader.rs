use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};

const DNA_BASES: u8 = 5;

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

pub fn read_dna_content(filename: String, k: u8) -> Result<HashMap<u32, Vec<u32>>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut mapping: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut index = 0 as u32;
    let mut buf = vec![0u8; 1024 * 1024];

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }

        process_chunk(&mut mapping, &buf, k, &mut index);
    }

    encode_deltas(&mut mapping);
    Ok(mapping)
}

fn process_chunk(map: &mut HashMap<u32, Vec<u32>>, buf: &[u8], k: u8, index: &mut u32) {
    if buf.len() < k as usize {
        return;
    }

    let mut kmer = 0 as u32;
    let power_table = build_power_cache(DNA_BASES, k);
    for i in 0..buf.len() - k as usize {
        let slice = &buf[i..i + k as usize];
        if i == 0 {
            for (j, &digit) in slice.iter().enumerate() {
                let index = (k as usize) - (j as usize) - 1;
                kmer += get_digit(digit) as u32 * power_table[index];
            }
        } else {
            let left_digit = get_digit(buf[i - 1]);
            let new_digit = get_digit(buf[i + k as usize - 1]);
            let last_power = power_table.last().unwrap();
            kmer = (kmer - left_digit * last_power) * DNA_BASES as u32 + new_digit;
        }
        map.entry(kmer).or_default().push(*index);
        *index += 1;
    }
}

fn encode_deltas(map: &mut HashMap<u32, Vec<u32>>) {
    for indices in map.values_mut() {
        for i in (1..indices.len()).rev() {
            indices[i] = indices[i] - indices[i - 1] as u32;
        }
    }
}

fn build_power_cache(length: u8, k: u8) -> Vec<u32> {
    let mut power_table = Vec::with_capacity(length as usize);
    power_table.push(1);
    for i in 1..length as usize {
        power_table.push(power_table[i - 1] * DNA_BASES as u32);
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
