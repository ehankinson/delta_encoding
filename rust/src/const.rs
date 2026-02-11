#[repr(u8)]
enum Codec {
    None     = 0,
    VarInt   = 1,
    BytePack = 2,
    Hybrid   = 3
}