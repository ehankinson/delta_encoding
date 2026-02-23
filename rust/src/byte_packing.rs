use crate::constants::Codec;
use crate::writer::write_header;

use std::mem::swap;
use std::sync::Arc;
use rustc_hash::FxHashMap;
use std::sync::mpsc::SyncSender;

pub fn byte_pack_encode(
    codec: &Codec,
    sender: &SyncSender<Vec<u8>>,
    word_freq: Arc<FxHashMap<u32, Vec<u32>>>,
) {
    let size = 8 * 1024 * 1024; // 8MB buffer
    let mut buffer = Vec::with_capacity(size);
    let mut payload = Vec::with_capacity(1024);
    let mut exceptions = Vec::with_capacity(128);

    let mut has_exception = false;

    for (_word, freq) in word_freq.iter() {
        payload.clear();
        exceptions.clear();

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

        let header = write_header(codec, has_exception, true);
        buffer.push(header);
        buffer.extend_from_slice(&freq.len().to_le_bytes());
        buffer.extend_from_slice(&freq[0].to_le_bytes());

        buffer.extend_from_slice(&payload.len().to_le_bytes());
        buffer.extend_from_slice(&payload);

        if has_exception {
            buffer.extend_from_slice(&exceptions.len().to_le_bytes());
            buffer.extend_from_slice(&exceptions);
        }

        if buffer.len() >= size {
            let mut new_buf = Vec::with_capacity(size);
            swap(&mut buffer, &mut new_buf);
            sender.send(new_buf).unwrap();
        }
    }

    if buffer.len() > 0 {
        sender.send(buffer).unwrap();
    }
}
