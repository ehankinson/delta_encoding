use crate::constants::DNA_BASES;

pub fn decode_dna_kmer(kmer: u32, k: u32) -> Vec<u8> {
    let mut dna_bytes = Vec::with_capacity(k as usize);

    let mut tmp = kmer;
    for i in 0..k {
        let byte = tmp % DNA_BASES;
        tmp /= DNA_BASES;
        dna_bytes[(k as usize) - (i as usize) - 1] = byte as u8;
    }

    dna_bytes
}