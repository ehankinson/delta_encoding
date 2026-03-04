use rustc_hash::FxHashMap;
use std::mem::swap;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;

use crate::constants::{Codec, EncodingInput, Posting, PostingData};
use crate::util::build_payload;

pub fn encode(
    input: EncodingInput,
    sender: &SyncSender<PostingData>,
    word_freq: Arc<FxHashMap<u32, Vec<u32>>>,
) {
    let size = 8 * 1024 * 1024; // 8MB buffer
    let mut payload = Vec::with_capacity(size);
    let mut byte_data = Vec::with_capacity(1024);
    let mut exceptions = Vec::with_capacity(128);

    for (term_id, freq) in word_freq.iter() {
        payload.clear();
        byte_data.clear();
        exceptions.clear();

        match input.codec {
            Codec::None => delta_encode(freq, &mut byte_data),
            Codec::VarInt => varint_encode(freq, &mut byte_data),
            Codec::BytePack => byte_pack_encode(freq, &mut byte_data, &mut exceptions),
            _ => panic!("Invalid codec"),
        }

        let has_exception = !exceptions.is_empty();

        let mut output_byte_data = Vec::new();
        swap(&mut byte_data, &mut output_byte_data);

        let mut output_exceptions = Vec::new();
        if has_exception {
            swap(&mut exceptions, &mut output_exceptions);
        }

        let posting_data = Posting {
            n: freq.len() as u32,
            base: freq[0],
            payload: output_byte_data,
            exceptions: if has_exception {
                Some(output_exceptions)
            } else {
                None
            },
        };

        build_payload(&input, posting_data, &mut payload);

        let mut output = Vec::new();
        swap(&mut output, &mut payload);

        let data = PostingData {
            term_id: *term_id,
            payload: output,
        };

        sender.send(data).unwrap();
    }
}

fn byte_pack_encode(freq: &[u32], byte_data: &mut Vec<u8>, exceptions: &mut Vec<u8>) {
    for &value in &freq[1..] {
        if value <= 255 {
            byte_data.push(value as u8);
        } else {
            byte_data.push(0);
            if value <= u16::MAX as u32 {
                let v = value as u16;
                exceptions.extend_from_slice(&v.to_le_bytes());
            }
            // For large values, store sentinel (0x0000) + 4-byte value.
            else {
                exceptions.extend_from_slice(&(0 as u16).to_le_bytes());
                exceptions.extend_from_slice(&value.to_le_bytes());
            }
        }
    }
}

fn delta_encode(freq: &[u32], byte_data: &mut Vec<u8>) {
    for &value in &freq[1..] {
        byte_data.extend_from_slice(&value.to_le_bytes());
    }
}

fn varint_encode(freq: &[u32], byte_data: &mut Vec<u8>) {
    for &value in freq.iter().skip(1) {
        let mut temp = value;
        while temp >= 0x80 {
            byte_data.push(((temp & 0x7F) | 0x80) as u8);
            temp >>= 7;
        }
        byte_data.push(temp as u8);
    }
}

pub(crate) fn encode_posting(codec: Codec, freq: &[u32], byte_data: &mut Vec<u8>, exceptions: &mut Vec<u8>) {
    match codec {
        Codec::None => delta_encode(freq, byte_data),
        Codec::VarInt => varint_encode(freq, byte_data),
        Codec::BytePack => byte_pack_encode(freq, byte_data, exceptions),
        _ => panic!("Invalid codec"),
    }
}
