mod reader;
mod writer;
mod constants;
mod util;
mod encoder;

use std::thread;
use std::sync::{Arc, mpsc};
use rustc_hash::FxHashMap;
use crate::{constants::{Kind, Codec, EncodingInput, PostingData}};

fn main() {
    let kmer_size = 5;
    let filename = "../data/dna/human_cleaned.txt".to_string();
    // let filename = "../data/books/frankenstein_or_the_modern_prometheus_by_mary_wollstonecraft_shelley_41445.txt".to_string();
    // let words = reader::read_book_content(filename).expect("failed to read input file");
    let start_time = std::time::Instant::now();
    let words = reader::read_dna_content(filename, kmer_size).expect("failed to read input file");
    let words = Arc::new(words);
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Time for reading the file took: {:#?}", duration);

    let byte_pack_input = EncodingInput {
        kind: Kind::DNA,
        codec: Codec::BytePack,
        kmer_size: Some(kmer_size)
    };
    let byte_pack_size = benchmark(byte_pack_input, Arc::clone(&words));

    let delta_encoding_input = EncodingInput {
        kind: Kind::DNA,
        codec: Codec::None,
        kmer_size: Some(kmer_size)
    };
    let delta_encoding_size = benchmark(delta_encoding_input, Arc::clone(&words));

    let varint_encoding_input = EncodingInput {
        kind: Kind::DNA,
        codec: Codec::VarInt,
        kmer_size: Some(kmer_size)
    };
    let varint_encoding_size = benchmark(varint_encoding_input, Arc::clone(&words));

    println!("The compression ratio is: {:?}", delta_encoding_size as f64 / byte_pack_size as f64);
    println!("The compression ratio for varint is: {:?}", delta_encoding_size as f64 / varint_encoding_size as f64);

}


fn benchmark(input: EncodingInput, word_freq: Arc<FxHashMap<u32, Vec<u32>>>) -> u64{
    //Could do it all in one match, but i dont want string creation part of the bench marking
    let (filename, path) = match input.codec {
        Codec::None => ("delta_encoding".to_string(), "delta_encoding_postings.bin"),
        Codec::VarInt => ("varint_encoding".to_string(), "varint_encoding_postings.bin"),
        Codec::BytePack => ("byte_pack".to_string(),"byte_pack_postings.bin"),
        Codec::Hybrid => ("hybrid_encoding".to_string(), "delta_encoding_postings.bin")
    };

    let start_time = std::time::Instant::now();
    let (sender, receiver) = mpsc::sync_channel::<PostingData>(8);
    let producer = thread::spawn(move || {
        encoder::encode(input, &sender, word_freq)
    });

    writer::writer(&filename, &receiver).unwrap();

    producer.join().unwrap();

    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("{}", "-".repeat(54));
    println!("Time for {:?} encoding took: {:#?}", filename, duration);
    let size  =  std::fs::metadata(path).unwrap().len();
    println!("{:?} has size {:.2}mb", filename, (size as f64 / (1024.0 * 1024.0)) );
    println!("{}", "-".repeat(54));
    println!();

    size
}
