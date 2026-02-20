#[repr(u8)]
pub enum Codec {
    None,
    VarInt,
    BytePack,
    Hybrid,
}

pub fn codec_to_bits(codec: &Codec) -> u8 {
    match codec {
        Codec::None => 0b00,
        Codec::VarInt => 0b01,
        Codec::BytePack => 0b10,
        Codec::Hybrid => 0b11,
    }
}

pub struct Posting {
    pub word: String,                // used for the dict.bin file
    pub n: u32,                      // length of the frequency vec (for the word)
    pub base: u32,                   // the literal word index like the frist occrance
    pub payload: Vec<u8>,            // the bytes for the delta encoding
    pub exceptions: Option<Vec<u8>>, // for bytpacking exceptions (that dont fit in 255)
}
