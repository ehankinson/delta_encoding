mod reader;
mod writer;
mod constants;
mod util;
mod encoder;
mod benchmark;
mod file_reader;

use std::thread;
use std::sync::{Arc, mpsc};
use rustc_hash::FxHashMap;
use crate::benchmark::hybrid_stim;
use crate::{constants::{Kind, Codec, EncodingInput, PostingData}};

fn main() {
    let kmer_size = 3;
    // let filename = "../data/dna/human_cleaned.txt".to_string();
    let filename = "../data/books/the_complete_works_of_william_shakespeare_by_william_shakespeare.txt".to_string();
    let start_time = std::time::Instant::now();

    let (word_freq, terms) = reader::read_book_content(filename).expect("failed to read input file");
    // let (word_freq, terms) = reader::read_dna_content(filename, kmer_size).expect("failed to read input file");
    let words = Arc::new(word_freq);
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    // println!("Time for reading the file took: {:#?}", duration);
    // println!("Number of terms: {}", terms.len());
    // println!("Average term length: {:.2}", terms.iter().map(|t| t.len()).sum::<usize>() as f64 / terms.len() as f64);
    // hybrid_stim(Arc::clone(&words));
    // benchmark::run_codec_benchmark(Codec::BytePack, words.clone());
    // println!("{}", "-".repeat(54));
    // benchmark::run_codec_benchmark(Codec::VarInt, words.clone());
    // println!("{}", "-".repeat(54));
    // benchmark::run_codec_benchmark(Codec::Hybrid, words.clone());
    let byte_pack_input = EncodingInput {
        kind: Kind::DNA,
        codec: Codec::BytePack,
        kmer_size: Some(kmer_size)
    };
    let byte_pack_size = run_encoding_pipeline_benchmark(byte_pack_input, Arc::clone(&words), &terms);

    let delta_encoding_input = EncodingInput {
        kind: Kind::DNA,
        codec: Codec::None,
        kmer_size: Some(kmer_size)
    };
    let delta_encoding_size = run_encoding_pipeline_benchmark(delta_encoding_input, Arc::clone(&words), &terms);

    let varint_encoding_input = EncodingInput {
        kind: Kind::DNA,
        codec: Codec::VarInt,
        kmer_size: Some(kmer_size)
    };
    let varint_encoding_size = run_encoding_pipeline_benchmark(varint_encoding_input, Arc::clone(&words), &terms);

    let hybrid_input = EncodingInput {
        kind: Kind::DNA,
        codec: Codec::Hybrid,
        kmer_size: Some(kmer_size)
    };
    let hybrid_encoding_size = run_encoding_pipeline_benchmark(hybrid_input, Arc::clone(&words), &terms);

    println!("The compression ratio for bytepacking is : {:?}", delta_encoding_size as f64 / byte_pack_size as f64);
    println!("The compression ratio for varint is: {:?}", delta_encoding_size as f64 / varint_encoding_size as f64);
    println!("The compression ratio for hybrid is: {:?}", delta_encoding_size as f64 / hybrid_encoding_size as f64);

    let pct = (delta_encoding_size as f64 / hybrid_encoding_size as f64) / (delta_encoding_size as f64 / varint_encoding_size as f64);
    println!("Hybrid beats varint by {:?} percentages", pct);

    println!("{}", "-".repeat(54));
    println!("Testing the File Reader...");
    
    let formats = ["delta_encoding", "varint_encoding", "byte_pack", "hybrid_encoding"];
    let search_term = "hamlet";

    for format in formats.iter() {
        let dict_filename = format!("{}_dict.bin", format);
        let postings_filename = format!("{}_postings.bin", format);
        
        let dict_start_time = std::time::Instant::now();
        match file_reader::load_dictionary(&dict_filename) {
            Ok(dictionary) => {
                let dict_duration = dict_start_time.elapsed();
                println!("{} -> Loaded dictionary ({} terms) in {:?}", format, dictionary.len(), dict_duration);
                
                let lookup_start_time = std::time::Instant::now();
                match file_reader::lookup_postings(search_term, &dictionary, &postings_filename) {
                    Ok(Some((codec, _has_exception, posting))) => {
                        let lookup_duration = lookup_start_time.elapsed();
                        println!("  Found '{}' -> Codec: {} | Occurrences: {} | Payload: {} bytes | Time: {:?}", 
                            search_term, codec as u8, posting.n, posting.payload.len(), lookup_duration);
                    },
                    Ok(None) => println!("  Term '{}' not found in dictionary.", search_term),
                    Err(e) => println!("  Error reading postings for '{}': {}", search_term, e),
                }
            },
            Err(e) => println!("Error loading dictionary {}: {}", dict_filename, e),
        }
    }
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
        Codec::Hybrid => ("hybrid_encoding".to_string(), "hybrid_encoding_postings.bin")
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
