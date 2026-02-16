mod reader;
mod writer;
mod constants;
mod byte_packing;
mod delta_encoding;

use crate::constants::Codec;

fn main() {
    let filename = "../data/books/the_complete_works_of_william_shakespeare_by_william_shakespeare.txt".to_string();
    let words = reader::read_content(filename).expect("failed to read input file");

    let byte_pack_codec = Codec::BytePack;
    let start_time = std::time::Instant::now();
    let byte_pack_postings = byte_packing::byte_pack_encode(&words);
    let byte_pack_filename = "byte_pack".to_string();
    let _ = writer::write_postings(byte_pack_codec, byte_pack_postings, byte_pack_filename);
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Time for byte packing encoding took: {:?}", duration);

    let delta_encoding_codec = Codec::None;
    let start_time = std::time::Instant::now();
    let delta_encoding_postings = delta_encoding::delta_encoding(&words);
    let delta_encoding_filename = "delta_encoding".to_string();
    let _ = writer::write_postings(delta_encoding_codec, delta_encoding_postings, delta_encoding_filename);
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Time for delta encoding encoding took: {:?}", duration);

    let byte_pack_size = std::fs::metadata("byte_pack_postings.bin").unwrap().len();
    let delta_encoding_size = std::fs::metadata("delta_encoding_postings.bin").unwrap().len();
    println!("Byte pack size: {:?}", byte_pack_size);
    println!("Delta encoding size: {:?}", delta_encoding_size);
    println!("The compression ratio is: {:?}", delta_encoding_size as f64 / byte_pack_size as f64);

}
