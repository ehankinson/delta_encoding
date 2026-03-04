use rustc_hash::FxHashMap;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;

use crate::constants::{Codec, EncodingInput, PostingData};
use crate::util::build_payload;

pub(crate) fn encode_posting(codec: Codec, freq: &[u32], byte_data: &mut Vec<u8>, exceptions: &mut Vec<u8>) {
    match codec {
        Codec::None => delta_encode(freq, byte_data),
        Codec::VarInt => varint_encode(freq, byte_data),
        Codec::BytePack => byte_pack_encode(freq, byte_data, exceptions),
        _ => panic!("Invalid codec"),
    }
}

pub fn encode(
    input: EncodingInput,
    sender: &SyncSender<PostingData>,
    word_freq: Arc<FxHashMap<u32, Vec<u32>>>,
) {
    let mut byte_data = Vec::with_capacity(1024);
    let mut exceptions = Vec::with_capacity(128);

    for (term_id, freq) in word_freq.iter() {
        if freq.is_empty() {
            continue;
        }

        byte_data.clear();
        exceptions.clear();

        match input.codec {
            Codec::None => byte_data.reserve((freq.len().saturating_sub(1)) * 4),
            Codec::VarInt => byte_data.reserve(freq.len().saturating_sub(1)),
            Codec::BytePack => {
                byte_data.reserve(freq.len().saturating_sub(1));
                exceptions.reserve(freq.len().saturating_sub(1) / 8);
            }
            _ => panic!("Invalid codec"),
        }

        encode_posting(input.codec, freq, &mut byte_data, &mut exceptions);

        let has_exception = !exceptions.is_empty();
        let mut payload = Vec::with_capacity(
            1 + 4 + 4 + std::mem::size_of::<usize>() + byte_data.len()
                + if has_exception {
                    std::mem::size_of::<usize>() + exceptions.len()
                } else {
                    0
                },
        );
        build_payload(
            &input,
            freq.len() as u32,
            freq[0],
            &byte_data,
            &exceptions,
            &mut payload,
        );

        let data = PostingData {
            term_id: *term_id,
            payload,
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
