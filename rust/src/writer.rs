use crate::constants::{codec_to_bits, Codec, PostingData};

use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::sync::mpsc::Receiver;

// The header will be a u8 and hold data like this:
// codec -> 2 bits
// has_exception -> 1 bit (needed for bytepacking)
// has_base -> 1 bit (needed for bytepacking)
pub fn write_header(codec: &Codec, has_exception: bool, has_base: bool) -> u8 {
    let mut header: u8 = 0b00000000;
    header |= codec_to_bits(codec) << 6;
    header |= (has_exception as u8) << 5;
    header |= (has_base as u8) << 4;

    header
}

// Write the values for decoding data
// term_id -> u32 (4 bytes)
// term_bytes_len -> u8 (1 byte) how many bytes are in the term_bytes
// term_bytes -> the actual bytes of the word
// offset -> where to start looking in the postings file (4 bytes)
fn build_dict(chunk: &PostingData, offset: &u32, term_bytes: &Vec<u8>) -> Result<Vec<u8>> {
    let mut dict = Vec::new();
    dict.extend_from_slice(&chunk.term_id.to_le_bytes());
    dict.extend_from_slice(&term_bytes.len().to_le_bytes());
    dict.extend_from_slice(&term_bytes);
    dict.extend_from_slice(&offset.to_le_bytes());

    Ok(dict)
}

pub fn writer(
    filename: &String,
    receiver: &Receiver<PostingData>,
    terms: &Vec<Vec<u8>>,
) -> Result<()> {
    let dict_file = File::create(format!("{}_dict.bin", filename)).unwrap();
    let postings_file = File::create(format!("{}_postings.bin", filename)).unwrap();

    let mut dict_writer = BufWriter::new(dict_file);
    let mut postings_writer = BufWriter::new(postings_file);

    let mut offset = 0;

    for chunk in receiver.iter() {
        let term_bytes = &terms[chunk.term_id as usize];
        let dict = build_dict(&chunk, &offset, term_bytes)?;
        dict_writer.write_all(&dict)?;
        postings_writer.write_all(&chunk.payload)?;
        offset += chunk.payload.len() as u32;
    }

    postings_writer.flush()?;

    Ok(())
}
