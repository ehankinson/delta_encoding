use std::sync::Arc;
use std::mem::swap;
use rustc_hash::FxHashMap;
use std::sync::mpsc::SyncSender;

use crate::util::build_payload;
use crate::util::decode_dna_kmer;
use crate::constants::{
    Kind, Codec, Posting, PostingData, EncodingInput
};


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
            Codec::None => delta_encode(&freq, &mut byte_data),
            Codec::VarInt => varint_encode(&freq, &mut byte_data),
            Codec::BytePack => byte_pack_encode(&freq, &mut byte_data, &mut exceptions),
            _ => panic!("Invalid codec"),
        }

        let has_exception = exceptions.len() > 0;

        let mut output_byte_data = Vec::new();
        swap(&mut byte_data, &mut output_byte_data);

        let mut output_exceptions = Vec::new();
        if has_exception {
            swap(&mut exceptions, &mut output_exceptions);
        }

        let posting_data = Posting {
            n: freq.len() as u32,
            base: freq[0] as u32,
            payload: output_byte_data,
            exceptions: if has_exception { Some(output_exceptions) } else { None },
        };

        build_payload(&input, posting_data, &mut payload);

        let mut term_bytes = Vec::new();

        if input.kind == Kind::DNA {
            if let Some(kmer_size) = input.kmer_size {
                term_bytes = decode_dna_kmer(*term_id, kmer_size);
            }

        }
        let mut output = Vec::new();
        swap(&mut output, &mut payload);

        let data = PostingData {
            term_id: *term_id,
            term_bytes: term_bytes,
            payload: output
        };

        sender.send(data).unwrap();
    }

}



fn byte_pack_encode(freq: &Vec<u32>,byte_data: &mut Vec<u8>, exceptions: &mut Vec<u8>) {
    for &value in &freq[1..] {
        if value <= 255 {
            byte_data.push(value as u8);
        } else {
            if value > u16::MAX as u32 {
                panic!("byte_pack exception overflow: {} does not fit in u16", value);
            }
            byte_data.push(0);
            exceptions.push((value & 0xFF) as u8);
            exceptions.push(((value >> 8) & 0xFF) as u8);
        }
    }
}



fn delta_encode(freq: &Vec<u32>, byte_data: &mut Vec<u8>) {
    for &value in &freq[1..] {
        byte_data.extend_from_slice(&value.to_le_bytes());
    }
}



fn varint_encode(freq: &Vec<u32>, byte_data: &mut Vec<u8>) {
    for &value in freq.iter().skip(1) {
        let mut temp = value;
        while temp >= 0x80 {
            byte_data.push(((temp & 0x7F) | 0x80) as u8);
            temp >>= 7;
        }
        byte_data.push(temp as u8);
    }
}
