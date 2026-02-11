use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

pub fn read_content(filename: String) -> std::io::Result<HashMap<String, Vec<i32>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
 
    let mut index = 0;
    let mut delta_encoding: HashMap<String, Vec<i32>> = HashMap::new();

    for lines in reader.lines(){
        let line = lines?;

        for word in line.split_whitespace(){
            
            let clean_word: String = word.chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(|c| c.to_lowercase())
            .collect();

             if clean_word.is_empty() {
                continue;
            }
             delta_encoding
            .entry(clean_word) 
            .or_default()
            .push(index as i32);

            index += 1;
        }
    }
    encode_deltas(&mut delta_encoding);
    Ok(delta_encoding)
}


pub fn encode_deltas(map: &mut HashMap<String, Vec<i32>>){
    for indices in map.values_mut(){
        for i in (1..indices.len()).rev(){
            indices[i] = indices[i] - indices[i-1];
        }
    }
}

// maybe 

//we make this function a life time. Meaning, we make a promise that the values of str wont be called after the word array
//dissapears. Instead of having a hashmap with more strings, we use their reference (pointers)
// pub fn generate_freq(word_array: Vec<String>) -> HashMap< String, Vec<i32>>{
//     let mut delta_encoding: HashMap<String, Vec<i32>> = HashMap::new();
//     //Enumerate is created from an iter. The reason why this is necessary because you need to explicitly define what
//     //you want to do. For example, a regular iter is a read only reference, iter_mut is a mutable reference
//     for(i, string) in word_array.into_iter().enumerate() {
//         delta_encoding
//             .entry(string)
//             .and_modify(|vec|{
//                 let last_pos = *vec.last().unwrap();
//                 vec.push(i as i32 - last_pos);
//             }) 
//             //makes the default value
//             .or_default()
//             .push(i as i32);
//     } 
//     delta_encoding
// }
