use crate::constants::{Codec, Posting};
use crate::writer::write_header;

use std::collections::HashMap;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;

pub fn byte_pack_encode(
    codec: &Codec,
    sender: &SyncSender<Vec<u8>>,
    word_freq: Arc<HashMap<u32, Vec<u32>>>,
) {
    let size = 8 * 1024 * 1024; // 8MB buffer
    let mut buffer = Vec::new();

    for (word, freq) in word_freq.iter() {
        let tail_len = freq.len().saturating_sub(1);
        let mut payload: Vec<u8> = Vec::with_capacity(tail_len);
        let mut exceptions: Option<Vec<u8>> = None;

        for &value in &freq[1..] {
            if value <= 255 {
                payload.push(value as u8);
            } else {
                payload.push(0);
                let ex = exceptions.get_or_insert_with(|| Vec::with_capacity(8));
                ex.push((value & 0xFF) as u8);
                ex.push(((value >> 8) & 0xFF) as u8);
            }
        }

        let header = write_header(codec, exceptions.is_some(), freq.len() > 0);
        buffer.push(header);
        buffer.extend_from_slice(&freq.len().to_le_bytes());
        buffer.extend_from_slice(&freq[0].to_le_bytes());
        buffer.extend_from_slice(&payload.len().to_le_bytes());
        buffer.extend_from_slice(&payload);
        if let Some(exceptions) = exceptions {
            buffer.extend_from_slice(&exceptions.len().to_le_bytes());
            buffer.extend_from_slice(&exceptions);
        }

        if buffer.len() >= size {
            sender.send(buffer).unwrap();
            buffer = Vec::new();
        }
    }
}
