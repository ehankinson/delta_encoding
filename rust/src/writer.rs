mod const;

// The header will be a u8 and hold data like this:
// codec -> 2 bits
// has_exception -> 1 bit (needed for bytepacking)
// has_base -> 1 bit (needed for bytepacking)
fn write_header(codec: Codec, has_exception: u8, has_base: u8) -> u8 {
    let mut header: u8 = 0b00000000;
    header |= (codec as u8) << 6;
    header |= has_exception << 5;
    header |= has_base << 4;
    
    header
}



pub fn write_postings(codec: Codec, word_freq: HashMap<String, Vec<i32>>) -> Vec<u8> {
    // build the data and exception list

    // build the header
    match codec {
        Codec::BytePack => write_header(codec, len(exceptions) > 0, true)
    }
}




// postings:
// header -> u8
// n -> u32
// base -> u32 // since we could have the first occurance at a number larger than 2^16 (65,536)
// payload_length -> u32
// payload -> Vec<u8>
// if has_exception:
//     exception_length -> u32
//     exceptions -> Vec<u16>