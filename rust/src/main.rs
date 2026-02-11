mod reader;

use std::time::Instant;

fn main() {
    let filename = "../data/books/the_complete_works_of_william_shakespeare_by_william_shakespeare.txt".to_string();
    let words = reader::read_content(filename);
    let start_time = Instant::now();
    let duration = start_time.elapsed();
    println!("Time taken to read file: {:?}", duration);
    println!("Length of words: {}", words.len());
    println!("First 10 words: {:?}", words.iter().take(10).collect::<Vec<&String>>());
    let end_time = Instant::now();
    let duration = end_time.elapsed();
    println!("Time taken to print words: {:?}", duration);
}