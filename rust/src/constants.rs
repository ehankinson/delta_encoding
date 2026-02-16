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
    pub word: String, // used for the dict.bin file
    pub n: u32,
    pub base: u32,
    pub payload: Vec<u8>,
    pub exceptions: Option<Vec<u8>>,
}
