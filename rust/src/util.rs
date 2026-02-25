use crate::constants::{EncodingInput, Posting, DNA_BASES};
use crate::writer::write_header;

pub fn build_payload(input: &EncodingInput, data: Posting, payload: &mut Vec<u8>) {
    let has_exception = data.exceptions.is_some();

    let header = write_header(&input.codec, has_exception, true);
    payload.push(header);
    payload.extend_from_slice(&data.n.to_le_bytes());
    payload.extend_from_slice(&data.base.to_le_bytes());

    payload.extend_from_slice(&data.payload.len().to_le_bytes());
    payload.extend_from_slice(&data.payload);

    if has_exception {
        let ex = data.exceptions.unwrap();
        payload.extend_from_slice(&ex.len().to_le_bytes());
        payload.extend_from_slice(&ex);
    }
}

pub fn decode_dna_kmer(kmer: u32, k: u32) -> Vec<u8> {
    let mut dna_bytes = vec![0; k as usize];

    let mut tmp = kmer;
    for i in 0..k {
        let digit = get_dna_char(tmp % DNA_BASES);
        tmp /= DNA_BASES;
        dna_bytes[(k as usize) - (i as usize) - 1] = digit;
    }

    dna_bytes
}

pub fn get_digit(byte: u8) -> u32 {
    match byte {
        b'A' => 0,
        b'C' => 1,
        b'G' => 2,
        b'T' => 3,
        b'N' => 4,
        _ => 0,
    }
}

pub fn get_dna_char(digit: u32) -> u8 {
    match digit {
        0 => b'A',
        1 => b'C',
        2 => b'G',
        3 => b'T',
        4 => b'N',
        _ => b'A',
    }
}
