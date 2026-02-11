mod reader;


fn main() {
    let filename = "../data/books/moby_dick_or_the_whale_by_herman_melville.txt".to_string();
    let words = reader::read_content(filename);
    println!("Length of words: {}", words.len());
    println!("First 10 words: {:?}", words.iter().take(10).collect::<Vec<&String>>());
}