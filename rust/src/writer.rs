use crate::constants::{codec_to_bits, Codec, Posting};

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

fn write_dict<W: Write>(writer: &mut W, posting: &Posting, offset: &u32) -> Result<()> {
    let len = posting.word.len() as u8; // The length of the word
    writer.write_all(&len.to_le_bytes())?;
    writer.write_all(posting.word.as_bytes())?;
    writer.write_all(&offset.to_le_bytes())?;

    Ok(())
}

pub fn writer(filename: &String, receiver: &Receiver<Vec<u8>>) -> Result<()> {
    // let dict_file = File::create(format!("{}_dict.bin", filename)).unwrap();
    let postings_file = File::create(format!("{}_postings.bin", filename)).unwrap();

    // let mut dict_writer = BufWriter::new(dict_file);
    let mut postings_writer = BufWriter::new(postings_file);

    for chunk in receiver.iter() {
        postings_writer.write_all(&chunk)?;
    }

    postings_writer.flush()?;

    Ok(())
}
