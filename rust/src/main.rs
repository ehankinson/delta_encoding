mod reader;
mod writer;
mod constants;
mod util;
mod encoder;
mod benchmark;

use std::thread;
use std::sync::{Arc, mpsc};
use rustc_hash::FxHashMap;
use crate::benchmark::hybrid_stim;
use crate::{constants::{Kind, Codec, EncodingInput, PostingData}};

fn main() {
    let kmer_size = 5;
    // let filename = "../data/dna/human_cleaned.txt".to_string();
    let filename = "../data/books/the_complete_works_of_william_shakespeare_by_william_shakespeare.txt".to_string();
    let start_time = std::time::Instant::now();

    let (word_freq, terms) = reader::read_book_content(filename).expect("failed to read input file");
    // let (word_freq, terms) = reader::read_dna_content(filename, kmer_size).expect("failed to read input file");
    let words = Arc::new(word_freq);
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Time for reading the file took: {:#?}", duration);
    println!("Number of terms: {}", terms.len());
    println!("Average term length: {:.2}", terms.iter().map(|t| t.len()).sum::<usize>() as f64 / terms.len() as f64);
    hybrid_stim(Arc::clone(&words));
    // benchmark::run_codec_benchmark(Codec::BytePack, words.clone());
    // println!("{}", "-".repeat(54));
    // benchmark::run_codec_benchmark(Codec::VarInt, words.clone());

    // let byte_pack_input = EncodingInput {
    //     kind: Kind::DNA,
    //     codec: Codec::BytePack,
    //     kmer_size: Some(kmer_size)
    // };
    // let byte_pack_size = run_encoding_pipeline_benchmark(byte_pack_input, Arc::clone(&words), &terms);

    // let delta_encoding_input = EncodingInput {
    //     kind: Kind::DNA,
    //     codec: Codec::None,
    //     kmer_size: Some(kmer_size)
    // };
    // let delta_encoding_size = run_encoding_pipeline_benchmark(delta_encoding_input, Arc::clone(&words), &terms);

    // let varint_encoding_input = EncodingInput {
    //     kind: Kind::DNA,
    //     codec: Codec::VarInt,
    //     kmer_size: Some(kmer_size)
    // };
    // let varint_encoding_size = run_encoding_pipeline_benchmark(varint_encoding_input, Arc::clone(&words), &terms);

    // println!("The compression ratio is: {:?}", delta_encoding_size as f64 / byte_pack_size as f64);
    // println!("The compression ratio for varint is: {:?}", delta_encoding_size as f64 / varint_encoding_size as f64);

}


fn run_encoding_pipeline_benchmark(
    input: EncodingInput,
    word_freq: Arc<FxHashMap<u32, Vec<u32>>>,
    terms: &[Vec<u8>],
) -> u64 {
    //Could do it all in one match, but i dont want string creation part of the bench marking
    let (filename, path) = match input.codec {
        Codec::None => ("delta_encoding".to_string(), "delta_encoding_postings.bin"),
        Codec::VarInt => ("varint_encoding".to_string(), "varint_encoding_postings.bin"),
        Codec::BytePack => ("byte_pack".to_string(),"byte_pack_postings.bin"),
        Codec::Hybrid => ("hybrid_encoding".to_string(), "delta_encoding_postings.bin")
    };

    let start_time = std::time::Instant::now();
    let (sender, receiver) = mpsc::sync_channel::<PostingData>(256);
    let producer = thread::spawn(move || {
        encoder::encode(input, &sender, word_freq)
    });

    writer::writer(&filename, &receiver, terms).unwrap();

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
