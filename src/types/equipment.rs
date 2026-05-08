use crate::types::item::Item;
use crate::types::serializable::BinarySerializable;
use crate::utils::SaveError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct Equipment {
    pub inventory_items: Vec<Item>,
    pub equipped_items: [i32; 31],
}

impl BinarySerializable for Equipment {
    fn read<R: Read>(reader: &mut R, version: i32) -> Result<Self, SaveError> {
        let inv_count = reader.read_i32::<LittleEndian>()?;
        if inv_count < 0 || inv_count > 100000 {
            return Err(SaveError::InvalidData(format!(
                "Invalid inventory count: {}",
                inv_count
            )));
        }
        let mut inventory_items = Vec::with_capacity(inv_count as usize);
        for _ in 0..inv_count {
            inventory_items.push(Item::read(reader, version)?);
        }

        let mut equipped_items = [-1; 31];
        for slot in &mut equipped_items {
            *slot = reader.read_i32::<LittleEndian>()?;
        }

        Ok(Equipment {
            inventory_items,
            equipped_items,
        })
    }

    fn write<W: Write>(&self, writer: &mut W, version: i32) -> Result<(), SaveError> {
        writer.write_i32::<LittleEndian>(self.inventory_items.len() as i32)?;
        for item in &self.inventory_items {
            item.write(writer, version)?;
        }
        for slot in &self.equipped_items {
            writer.write_i32::<LittleEndian>(*slot)?;
        }
        Ok(())
    }
}
