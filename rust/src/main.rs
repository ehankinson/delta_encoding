mod reader;
mod writer;
mod constants;
mod byte_packing;
mod delta_encoding;
mod varint;

use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;
use crate::{constants::Codec, constants::Posting};

fn main() {
    let filename = "../data/dna/human_cleaned.txt".to_string();
    // let words = reader::read_book_content(filename).expect("failed to read input file");
    let start_time = std::time::Instant::now();
    let words = reader::read_dna_content(filename, 5).expect("failed to read input file");
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);

    let (chunk_generator, chunk_writer) = mpsc::sync_channel::<Posting>(8);



    println!("Time for reading the file took: {:#?}", duration);
    let byte_pack_size = benchmark(Codec::BytePack, &words);
    let delta_encoding_size = benchmark(Codec::None, &words);
    let varint_encoding_size = benchmark(Codec::VarInt, &words);
    println!("The compression ratio is: {:?}", delta_encoding_size as f64 / byte_pack_size as f64);
    println!("The compression ratio for varint is: {:?}", delta_encoding_size as f64 / varint_encoding_size as f64);

}


fn benchmark(encoder: Codec, word_freq: &HashMap<String, Vec<u32>> ) -> u64{
    //Could do it all in one match, but i dont want string creation part of the bench marking
    let (filename, path) = match encoder {
        Codec::None => ("delta_encoding".to_string(), "delta_encoding_postings.bin"),
        Codec::VarInt => ("varint_encoding".to_string(), "varint_encoding_postings.bin"),
        Codec::BytePack => ("byte_pack".to_string(),"byte_pack_postings.bin"),
        Codec::Hybrid => ("hybrid_encoding".to_string(), "delta_encoding_postings.bin")
    };

    let start_time = std::time::Instant::now();
    let _ = match encoder {
        Codec::None => 
            writer::writer(encoder, delta_encoding::delta_encoding(word_freq), &filename),

        Codec::VarInt => 
            writer::writer(encoder, varint::varint_encode(word_freq), &filename),

        Codec::BytePack => 
            writer::writer(encoder, byte_packing::byte_pack_encode(word_freq), &filename),

        Codec::Hybrid => //defaulting to delta encode :p
            writer::writer(encoder, delta_encoding::delta_encoding(word_freq), &filename),
    };
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
