use crate::constants::{codec_to_bits, Codec, Posting};
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
    posting: &Posting,
    codec: &Codec,
    offset: &mut u32,
) -> Result<()> {
    let exceptions = posting.exceptions.as_deref().filter(|ex| !ex.is_empty());
    let has_exception = posting.exceptions.is_some();
    let header = write_header(codec, has_exception, posting.base > 0);

    writer.write_all(&header.to_le_bytes())?;
    writer.write_all(&posting.n.to_le_bytes())?;
    writer.write_all(&posting.base.to_le_bytes())?;
    writer.write_all(&(posting.payload.len() as u32).to_le_bytes())?;
    writer.write_all(&posting.payload)?;

    *offset += 13 + posting.payload.len() as u32;

    if let Some(exceptions) = exceptions {
        let excep_len = exceptions.len() as u32;
        writer.write_all(&excep_len.to_le_bytes())?;
        writer.write_all(exceptions)?;
        *offset += 4 + excep_len * 2
    }

    Ok(())
}

fn write_dict<W: Write>(writer: &mut W, posting: &Posting, offset: &u32) -> Result<()> {
    let len = posting.word.len() as u8; // The length of the word
    writer.write_all(&len.to_le_bytes())?;
    writer.write_all(posting.word.as_bytes())?;
    writer.write_all(&offset.to_le_bytes())?;

    Ok(())
}

pub fn write_postings(codec: Codec, postings: Vec<Posting>, filename: &String) -> Result<()> {
    let dict_file = File::create(format!("{}_dict.bin", filename)).unwrap();
    let postings_file = File::create(format!("{}_postings.bin", filename)).unwrap();

    let mut dict_writer = BufWriter::new(dict_file);
    let mut postings_writer = BufWriter::new(postings_file);

    let mut offset: u32 = 0;
    for posting in postings.iter() {
        write_posting(&mut postings_writer, posting, &codec, &mut offset)?;
        write_dict(&mut dict_writer, posting, &offset)?;
    }
    dict_writer.flush()?;
    postings_writer.flush()?;

    Ok(())
}
