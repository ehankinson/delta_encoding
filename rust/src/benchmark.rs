use rustc_hash::FxHashMap;
use std::cmp;
use std::collections::HashMap;
use std::sync::Arc;

use crate::constants::Codec;
use crate::encoder::encode_posting;
pub fn hybrid_stim(word_freq: Arc<FxHashMap<u32, Vec<u32>>>) {
    let mut temp: HashMap<i32, i32> = HashMap::new();
    let mut ethan_map: HashMap<i32, Vec<f64>> = HashMap::new();

    let mut b_payload = Vec::new();
    let mut b_exceptions = Vec::new();
    let mut v_payload = Vec::new();

    let mut over = 0;
    let mut under = 0;

    let mut varint_win = 0;
    let mut bytepack_win = 0;
    for freq in word_freq.values() {
        b_payload.clear();
        b_exceptions.clear();
        v_payload.clear();
        if freq.len() == 1 {
            continue;
        }

        encode_posting(Codec::BytePack, freq, &mut b_payload, &mut b_exceptions);
        encode_posting(Codec::VarInt, freq, &mut v_payload, &mut Vec::new());
        let count = freq.iter().skip(1).filter(|&x| *x <= 255).count();
        let percentage: f64 = count as f64 / freq.len() as f64;

        // 1. Store the lengths in variables to make the code easier to read
        let b_len = b_payload.len() + b_exceptions.len();
        let v_len = v_payload.len();

        // 2. Use abs_diff()! It safely finds the absolute difference as a usize
        let diff = b_len.abs_diff(v_len);

        // 3. Calculate the percentage, casting directly to f64
        let pct = diff as f64 / cmp::max(b_len, v_len) as f64;
        if (pct <= 0.1) {
            if b_len < v_len {
                bytepack_win += 1;
            } else {
                varint_win += 1;
            }

            // 1. entry(1) finds the key or gets ready to make it
            // 2. or_insert_with(Vec::new) creates a new Vec if it was missing
            // 3. It returns a MUTABLE reference, allowing you to instantly .push()
            ethan_map.entry(1).or_insert_with(Vec::new).push(percentage);
        } else if ((b_payload.len() + b_exceptions.len()) < v_payload.len()) {
            ethan_map.entry(0).or_insert_with(Vec::new).push(percentage);
        } else {
            ethan_map.entry(2).or_insert_with(Vec::new).push(percentage);
        }
        // println!("The total count was {:?} and percentage of thing is {:?}", count, (((count as f64 / freq.len() as f64))))
    }
    println!("{:?}", temp);
    println!("--- Ethan Map Stats ---");
    // Loop through each key (0, 1, 2) and its vector of percentages
    for (key, vec) in &ethan_map {
        let count = vec.len();

        // Calculate the sum of all f64s in the vector.
        // We have to explicitly tell Rust the sum is an f64.
        let sum: f64 = vec.iter().sum();

        // Avoid dividing by zero (even though your vectors shouldn't be empty)
        let average = if count > 0 { sum / count as f64 } else { 0.0 };

        // Map the keys to human-readable names for your output
        let category_name = match key {
            0 => "BytePack Win",
            1 => "Tie / Close (< 10% diff)",
            2 => "VarInt Win",
            _ => "Unknown",
        };

        println!(
            "[{}] Count: {}, Average non-zero percentage: {:.4}",
            category_name, count, average
        );
    }

    println!(
        "For the ties, Bytepack win {:?} and varint win {:?}",
        bytepack_win, varint_win
    );
    // println!("Total number of words over 255 {:?} and under 255 {:?}", over, under);
    // println!("Bytepack win {:?} and varint win {:?}", bytepack_win, varint_win);
}

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
        Codec::BytePack => "byte_pack_encoding",
        Codec::Hybrid => "hybrid_encoding",
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
    println!(
        "Encode speed: {:.2} Millions ints/sec",
        millions_ints_per_sec
    );
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
