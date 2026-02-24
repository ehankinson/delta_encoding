use crate::constants::{EncodingInput, Kind, PostingData};
use crate::writer::write_header;
use crate::util::decode_dna_kmer;

use std::mem::swap;
use std::sync::Arc;
use rustc_hash::FxHashMap;
use std::sync::mpsc::SyncSender;

pub fn byte_pack_encode(
    input: EncodingInput,
    sender: &SyncSender<PostingData>,
    word_freq: Arc<FxHashMap<u32, Vec<u32>>>,
) {
    let size = 8 * 1024 * 1024; // 8MB buffer
    let mut posting_data = Vec::with_capacity(size);
    let mut payload = Vec::with_capacity(1024);
    let mut exceptions = Vec::with_capacity(128);

    for (term_id, freq) in word_freq.iter() {
        posting_data.clear();
        payload.clear();
        exceptions.clear();
        let mut has_exception = false;

        for &value in &freq[1..] {
            if value <= 255 {
                payload.push(value as u8);
            } else {
                payload.push(0);
                has_exception = true;
                exceptions.push((value & 0xFF) as u8);
                exceptions.push(((value >> 8) & 0xFF) as u8);
            }
        }

        let header = write_header(&input.codec, has_exception, true);
        posting_data.push(header);
        posting_data.extend_from_slice(&freq.len().to_le_bytes());
        posting_data.extend_from_slice(&freq[0].to_le_bytes());

        posting_data.extend_from_slice(&payload.len().to_le_bytes());
        posting_data.extend_from_slice(&payload);

        if has_exception {
            posting_data.extend_from_slice(&exceptions.len().to_le_bytes());
            posting_data.extend_from_slice(&exceptions);
        }

        let mut term_bytes = Vec::new();

        if input.kind == Kind::DNA {
            if let Some(kmer_size) = input.kmer_size {
                term_bytes = decode_dna_kmer(*term_id, kmer_size);
            }

        }
        let mut output = Vec::new();
        swap(&mut output, &mut posting_data);

        let data = PostingData {
            term_id: *term_id,
            term_bytes: term_bytes,
            payload: output
        };

        sender.send(data).unwrap();
    }
}
