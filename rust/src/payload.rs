use std::io::{Result, Write};

pub trait PayloadElement: Copy {
    fn write_one<W: Write>(self, writer: &mut W) -> Result<()>;
}

impl PayloadElement for u8 {
    fn write_one<W: Write>(self, w: &mut W) -> Result<()> {
        w.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl PayloadElement for i32 {
    fn write_one<W: Write>(self, w: &mut W) -> Result<()> {
        w.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}
