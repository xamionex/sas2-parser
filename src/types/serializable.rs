use crate::utils::SaveError;
use std::io::{Read, Write};

pub trait BinarySerializable: Sized {
    fn read<R: Read>(reader: &mut R, version: i32) -> Result<Self, SaveError>;
    fn write<W: Write>(&self, writer: &mut W, version: i32) -> Result<(), SaveError>;
}
