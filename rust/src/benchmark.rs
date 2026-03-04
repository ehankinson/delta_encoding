use rustc_hash::FxHashMap;
use std::sync::Arc;

use crate::constants::Codec;
use crate::encoder::encode_posting;

pub fn run_codec_benchmark(input: Codec, word_freq: Arc<FxHashMap<u32, Vec<u32>>>) {
    let total_postings_list = word_freq.len();
    let total_ints = word_freq
        .values()
        .map(|freq| freq.len().saturating_sub(1))
        .sum::<usize>();

    let avg_posting_length = if total_postings_list == 0 {
        0.0
    } else {
        total_ints as f64 / total_postings_list as f64
    };

    let max_gap = word_freq
        .values()
        .map(|freq| freq.iter().skip(1).max().copied().unwrap_or(0))
        .max()
        .unwrap_or(0);

    let mut bucket_half_byte = 0usize; // <= 15
    let mut bucket_1_byte = 0usize; // 16..=255
    let mut bucket_2_byte = 0usize; // 256..=65535
    let mut bucket_4_byte = 0usize; // > 65535

    for freq in word_freq.values() {
        for &value in freq.iter().skip(1) {
            if value <= 15 {
                bucket_half_byte += 1;
            } else if value <= u8::MAX as u32 {
                bucket_1_byte += 1;
            } else if value <= u16::MAX as u32 {
                bucket_2_byte += 1;
            } else {
                bucket_4_byte += 1;
            }
        }
    }

    let mut total_bytes = 0usize;
    let mut total_exceptions = 0usize;

    let mut byte_data = Vec::with_capacity(1024);
    let mut exceptions = Vec::with_capacity(128);

    // Warm up once to reduce cold-start effects on timing.
    for freq in word_freq.values() {
        byte_data.clear();
        exceptions.clear();
        encode_posting(input, freq, &mut byte_data, &mut exceptions);
    }

    let start = std::time::Instant::now();
    for freq in word_freq.values() {
        byte_data.clear();
        exceptions.clear();
        encode_posting(input, freq, &mut byte_data, &mut exceptions);

        total_bytes += byte_data.len() + exceptions.len();
        total_exceptions += exceptions.len();
    }
    let total_encode_ns = start.elapsed().as_nanos();

    let bytes_per_int = if total_ints == 0 {
        0.0
    } else {
        total_bytes as f64 / total_ints as f64
    };

    let compression_ratio = if total_bytes == 0 {
        0.0
    } else {
        (total_ints * 4) as f64 / total_bytes as f64
    };

    let string_codec = match input {
        Codec::None => "delta_encoding",
        Codec::VarInt => "varint_encoding",
        Codec::BytePack => "byte_pack",
        _ => panic!("Invalid codec"),
    };

    let ns_per_int = if total_ints == 0 {
        0.0
    } else {
        total_encode_ns as f64 / total_ints as f64
    };

    let millions_ints_per_sec = if ns_per_int == 0.0 {
        0.0
    } else {
        (1_000_000_000.0 / ns_per_int) / 1_000_000.0
    };

    let pct = |count: usize| -> f64 {
        if total_ints == 0 {
            0.0
        } else {
            (count as f64 * 100.0) / total_ints as f64
        }
    };

    println!("The codec used is: {}", string_codec);
    println!("Total postings list: {}", total_postings_list);
    println!("Total ints: {}", total_ints);
    println!("Average posting length: {:.2}", avg_posting_length);
    println!("Max gap: {}", max_gap);
    println!("Bytes per int: {:.2}", bytes_per_int);
    println!("Compression ratio: {:.2}", compression_ratio);

    if input == Codec::BytePack {
        println!("Exceptions bytes: {}", total_exceptions);
    }

    println!("Encode speed: {:.2} ns/int", ns_per_int);
    println!("Encode speed: {:.2} Millions ints/sec", millions_ints_per_sec);
    println!("Value-size histogram:");
    println!(
        "  <= 15      (~0.5B): {:>10} ({:>6.2}%)",
        bucket_half_byte,
        pct(bucket_half_byte)
    );
    println!(
        "  16..=255   (1B)   : {:>10} ({:>6.2}%)",
        bucket_1_byte,
        pct(bucket_1_byte)
    );
    println!(
        "  256..=65535(2B)   : {:>10} ({:>6.2}%)",
        bucket_2_byte,
        pct(bucket_2_byte)
    );
    println!(
        "  > 65535    (4B)   : {:>10} ({:>6.2}%)",
        bucket_4_byte,
        pct(bucket_4_byte)
    );
}
