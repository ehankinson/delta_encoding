mod reader;
mod writer;
mod constants;
mod byte_packing;

use crate::constants::Codec;

fn main() {
    let filename = "../data/books/the_complete_works_of_william_shakespeare_by_william_shakespeare.txt".to_string();
    let words = reader::read_content(filename).expect("failed to read input file");
    let codec = Codec::BytePack;
    let _ = writer::write_postings(codec, words);

}
