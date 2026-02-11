mod reader;

use std::time::Instant;

fn main() {
    let filename = "../data/books/the_complete_works_of_william_shakespeare_by_william_shakespeare.txt".to_string();
    //we can handle the Result later. Lazy rn.
    let words = reader::read_content(filename).unwrap();
  
    let start_time = Instant::now();
    let duration = start_time.elapsed();
    println!("Time taken to read file: {:?}", duration);
    // println!("Length of words: {}", words.len());
    // println!("First 10 words: {:?}", words.iter().take(10).collect::<Vec<&String>>());
    //i asked gpt to make 
    words.iter().take(2).for_each(|(w, v)| println!("{w} -> {:?}", v));
    let end_time = Instant::now();
    let duration = end_time.elapsed();
    println!("Time taken to print words: {:?}", duration);
}