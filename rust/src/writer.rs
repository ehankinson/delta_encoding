use crate::byte_packing;
use crate::constants::{codec_to_bits, BytePosting, Codec};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Result, Write};

// The header will be a u8 and hold data like this:
// codec -> 2 bits
// has_exception -> 1 bit (needed for bytepacking)
// has_base -> 1 bit (needed for bytepacking)
fn write_header(codec: &Codec, has_exception: bool, has_base: bool) -> u8 {
    let mut header: u8 = 0b00000000;
    header |= codec_to_bits(codec) << 6;
    header |= (has_exception as u8) << 5;
    header |= (has_base as u8) << 4;

    header
}

fn write_posting<W: Write>(
    writer: &mut W,
    posting: &BytePosting,
    codec: &Codec,
    offset: &mut u32,
) -> Result<()> {
    let has_exception = posting.exceptions.len() > 0;
    let header = write_header(codec, has_exception, posting.base > 0);

    writer.write_all(&header.to_le_bytes())?;
    writer.write_all(&posting.n.to_le_bytes())?;
    writer.write_all(&posting.base.to_le_bytes())?;
    writer.write_all(&(posting.payload.len() as u32).to_le_bytes())?;

    for &value in posting.payload.iter() {
        writer.write_all(&value.to_le_bytes())?;
    }

    *offset += 13 + posting.payload.len() as u32;

    if has_exception {
        writer.write_all(&(posting.exceptions.len() as u32).to_le_bytes())?;
        for &value in posting.exceptions.iter() {
            writer.write_all(&value.to_le_bytes())?;
        }
        *offset += 4 + posting.exceptions.len() as u32 * 2;
    }

    writer.flush()?;
    Ok(())
}

fn write_dict<W: Write>(writer: &mut W, postings: &BytePosting, offset: &u32) -> Result<()> {
    let len = postings.word.len() as u8; // The length of the word
    writer.write_all(&len.to_le_bytes())?;
    writer.write_all(postings.word.as_bytes())?;
    writer.write_all(&offset.to_le_bytes())?;

    writer.flush()?;
    Ok(())
}

pub fn write_postings(codec: Codec, word_freq: HashMap<String, Vec<i32>>) -> Result<()> {
    let postings: Vec<BytePosting> = byte_packing::byte_pack_encode(word_freq);

    let dict_file = File::create("dict.bin").unwrap();
    let postings_file = File::create("postings.bin").unwrap();

    let mut dict_writer = BufWriter::new(dict_file);
    let mut postings_writer = BufWriter::new(postings_file);

    let mut offset: u32 = 0;
    for posting in postings.iter() {
        write_posting(&mut postings_writer, posting, &codec, &mut offset)?;
        write_dict(&mut dict_writer, posting, &offset)?;
    }

    Ok(())
}
