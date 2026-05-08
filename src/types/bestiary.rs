use crate::types::serializable::BinarySerializable;
use crate::utils::SaveError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

pub const TOTAL_DROPS: usize = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct BestiaryBeast {
    pub kills: i32,
    pub deaths: i32,
    pub drops: [bool; TOTAL_DROPS],
}

impl BinarySerializable for BestiaryBeast {
    fn read<R: Read>(reader: &mut R, _version: i32) -> Result<Self, SaveError> {
        let kills = reader.read_i32::<LittleEndian>()?;
        let deaths = reader.read_i32::<LittleEndian>()?;
        let mut drops = [false; TOTAL_DROPS];
        for d in &mut drops {
            *d = reader.read_u8()? != 0;
        }
        Ok(BestiaryBeast {
            kills,
            deaths,
            drops,
        })
    }

    fn write<W: Write>(&self, writer: &mut W, _version: i32) -> Result<(), SaveError> {
        writer.write_i32::<LittleEndian>(self.kills)?;
        writer.write_i32::<LittleEndian>(self.deaths)?;
        for d in &self.drops {
            writer.write_u8(if *d { 1 } else { 0 })?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bestiary {
    pub beasts: Vec<BestiaryBeast>,
}

impl BinarySerializable for Bestiary {
    fn read<R: Read>(reader: &mut R, version: i32) -> Result<Self, SaveError> {
        let beast_count = reader.read_i32::<LittleEndian>()?;
        if beast_count < 0 || beast_count > 10000 {
            return Err(SaveError::InvalidData(format!(
                "Invalid bestiary count: {} (likely corrupted or modded format)",
                beast_count
            )));
        }
        let mut beasts = Vec::with_capacity(beast_count as usize);
        for _ in 0..beast_count {
            beasts.push(BestiaryBeast::read(reader, version)?);
        }
        Ok(Bestiary { beasts })
    }

    fn write<W: Write>(&self, writer: &mut W, version: i32) -> Result<(), SaveError> {
        writer.write_i32::<LittleEndian>(self.beasts.len() as i32)?;
        for beast in &self.beasts {
            beast.write(writer, version)?;
        }
        Ok(())
    }
}
