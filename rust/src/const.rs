#[repr(u8)]
enum Codec {
    None     = 0,
    VarInt   = 1,
    BytePack = 2,
    Hybrid   = 3
}

struct Posting {
    n: u32,
    base: u32,
    payload_length: u32,
    payload: Vec<u8>,
    exception_length: u32,
    exceptions: Vec<u16>
}