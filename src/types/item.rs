use crate::types::serializable::BinarySerializable;
use crate::utils::SaveError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub loot_idx: i32,
    pub count: i32,
    pub upgrade: i32,
    pub stock_piled: bool,
    // Mod-only fields
    pub artifact_seed: i32,
    pub item_version: i32,
    pub rarity: i32,
}

impl BinarySerializable for Item {
    fn read<R: Read>(reader: &mut R, version: i32) -> Result<Self, SaveError> {
        let loot_idx = reader.read_i32::<LittleEndian>()?;
        let count = reader.read_i32::<LittleEndian>()?;
        let upgrade = reader.read_i32::<LittleEndian>()?;
        let stock_piled = reader.read_u8()? != 0;

        let (artifact_seed, item_version, rarity) = if version >= 20 {
            let artifact_seed = reader.read_i32::<LittleEndian>()?;
            let item_version = reader.read_i32::<LittleEndian>()?;
            // Rarity only present if salt-guard
            let rarity = reader.read_i32::<LittleEndian>()?;
            (artifact_seed, item_version, rarity)
        } else {
            (-1, 0, 1) // Defaults for vanilla
        };

        Ok(Item {
            loot_idx,
            count,
            upgrade,
            stock_piled,
            artifact_seed,
            item_version,
            rarity,
        })
    }

    fn write<W: Write>(&self, writer: &mut W, version: i32) -> Result<(), SaveError> {
        writer.write_i32::<LittleEndian>(self.loot_idx)?;
        writer.write_i32::<LittleEndian>(self.count)?;
        writer.write_i32::<LittleEndian>(self.upgrade)?;
        writer.write_u8(if self.stock_piled { 1 } else { 0 })?;

        if version >= 20 {
            writer.write_i32::<LittleEndian>(self.artifact_seed)?;
            writer.write_i32::<LittleEndian>(self.item_version)?;
            writer.write_i32::<LittleEndian>(self.rarity)?;
        }

        Ok(())
    }
}
