use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};
use crate::constants::{Posting, Codec};

/// Reads the dictionary file sequentially and constructs an in-memory hash map.
/// The dictionary file structure for each term is:
/// [term_id: 4 bytes] [term_bytes_len: 4 bytes] [term_bytes: variable] [offset: 4 bytes]
/// We read `term_id` merely to advance the pointer, converting the byte array into a String,
/// and mapping that string to its respective byte offset in the postings file.
pub fn load_dictionary(dict_filename: &str) -> Result<HashMap<String, u32>> {
    let mut file = File::open(dict_filename)?;
    let mut dictionary = HashMap::new();
    let mut u32_buf = [0u8; 4];

    loop {
        let bytes_read = file.read(&mut u32_buf)?;
        if bytes_read == 0 {
            break;
        }

        file.read_exact(&mut u32_buf)?;
        let term_bytes_len = u32::from_le_bytes(u32_buf) as usize;

        let mut term_bytes = vec![0u8; term_bytes_len];
        file.read_exact(&mut term_bytes)?;
        let term = String::from_utf8_lossy(&term_bytes).into_owned();

        file.read_exact(&mut u32_buf)?;
        let offset = u32::from_le_bytes(u32_buf);

        dictionary.insert(term, offset);
    }

    Ok(dictionary)
}

fn bits_to_codec(bits: u8) -> Codec {
    match bits {
        0b00 => Codec::None,
        0b01 => Codec::VarInt,
        0b10 => Codec::BytePack,
        0b11 => Codec::Hybrid,
        _ => Codec::None,
    }
}

/// Retrieves the raw encoded postings for a specific term using O(1) file seeking.
/// First, it checks the dictionary for the term's byte offset. If found, it seeks directly
/// to that byte in the postings file. It then unpacks the 1-byte header to extract the codec
/// and exception flags, followed by the variable length postings payload and exceptions.
pub fn lookup_postings(
    term: &str, 
    dictionary: &HashMap<String, u32>, 
    postings_filename: &str
) -> Result<Option<(Codec, bool, Posting)>> {
    
    let offset = match dictionary.get(term) {
        Some(&off) => off,
        None => return Ok(None), 
    };

    let mut file = File::open(postings_filename)?;
    file.seek(SeekFrom::Start(offset as u64))?;

    let mut u32_buf = [0u8; 4];
    let mut header_buf = [0u8; 1];
    file.read_exact(&mut header_buf)?;
    let header = header_buf[0];

    let codec_bits = (header >> 6) & 0b11;
    let codec = bits_to_codec(codec_bits);
    let has_exception = ((header >> 5) & 1) == 1;

    file.read_exact(&mut u32_buf)?;
    let n = u32::from_le_bytes(u32_buf);

    file.read_exact(&mut u32_buf)?;
    let base = u32::from_le_bytes(u32_buf);

    file.read_exact(&mut u32_buf)?;
    let encoded_len = u32::from_le_bytes(u32_buf) as usize;

    let mut payload = vec![0u8; encoded_len];
    file.read_exact(&mut payload)?;

    let mut exceptions = None;
    if has_exception {
        file.read_exact(&mut u32_buf)?;
        let exceptions_len = u32::from_le_bytes(u32_buf) as usize;

        let mut exc_buf = vec![0u8; exceptions_len];
        file.read_exact(&mut exc_buf)?;
        exceptions = Some(exc_buf);
    }

    Ok(Some((codec, has_exception, Posting {
        n,
        base,
        payload,
        exceptions,
    })))
}