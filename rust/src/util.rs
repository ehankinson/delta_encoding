use crate::constants::{EncodingInput, DNA_BASES};
use crate::writer::write_header;

pub fn build_payload(
    input: &EncodingInput,
    n: u32,
    base: u32,
    encoded: &[u8],
    exceptions: &[u8],
    payload: &mut Vec<u8>,
) {
    let has_exception = !exceptions.is_empty();

    let header = write_header(&input.codec, has_exception, true);
    payload.push(header);
    payload.extend_from_slice(&n.to_le_bytes());
    payload.extend_from_slice(&base.to_le_bytes());

    payload.extend_from_slice(&encoded.len().to_le_bytes());
    payload.extend_from_slice(encoded);

    if has_exception {
        payload.extend_from_slice(&exceptions.len().to_le_bytes());
        payload.extend_from_slice(exceptions);
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
