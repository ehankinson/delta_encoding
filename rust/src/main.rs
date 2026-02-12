mod reader;
mod writer;
mod constants;
mod byte_packing;

use crate::constants::Codec;

fn main() {
    let start_time = std::time::Instant::now();
    let filename = "../data/books/the_complete_works_of_william_shakespeare_by_william_shakespeare.txt".to_string();
    let words = reader::read_content(filename).expect("failed to read input file");
    let codec = Codec::BytePack;
    let _ = writer::write_postings(codec, words);
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Time taken: {:?}", duration);

}
