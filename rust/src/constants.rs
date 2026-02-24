pub const DNA_BASES: u32 = 5;

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

#[derive(PartialEq)]
pub enum Kind {
    DNA,
    BOOK,
    HTML
}

pub struct Posting {
    pub word: String,                // used for the dict.bin file
    pub n: u32,                      // length of the frequency vec (for the word)
    pub base: u32,                   // the literal word index like the frist occrance
    pub payload: Vec<u8>,            // the bytes for the delta encoding
    pub exceptions: Option<Vec<u8>>, // for bytpacking exceptions (that dont fit in 255)
}

pub struct PostingData {
    pub term_id: u32,
    pub term_bytes: Vec<u8>,
    pub payload: Vec<u8>
}

pub struct EncodingInput {
    pub kind: Kind,
    pub codec: Codec,
    pub kmer_size: Option<u32>,
}