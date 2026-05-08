use crate::types::serializable::BinarySerializable;
use crate::utils::SaveError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub level: i32,
    pub stats: [i32; 9],
    pub xp: i64,
    pub silver: i64,
    pub dropped_xp: i64,
    pub dropped_xp_area: i32,
    pub dropped_xp_vec: (f32, f32),
    pub time_played: f64,
    pub hazeburnt: bool,
    pub item_class: [i32; 40],
    pub tree_unlocks: [i32; 500],
    pub class_unlocks: [i32; 3],
}

impl BinarySerializable for Stats {
    fn read<R: Read>(reader: &mut R, _version: i32) -> Result<Self, SaveError> {
        let level = reader.read_i32::<LittleEndian>()?;
        let mut stats = [0; 9];
        for s in &mut stats {
            *s = reader.read_i32::<LittleEndian>()?;
        }
        let xp = reader.read_i64::<LittleEndian>()?;
        let silver = reader.read_i64::<LittleEndian>()?;
        let dropped_xp = reader.read_i64::<LittleEndian>()?;
        let dropped_xp_area = reader.read_i32::<LittleEndian>()?;
        let dx = reader.read_f32::<LittleEndian>()?;
        let dy = reader.read_f32::<LittleEndian>()?;
        let time_played = reader.read_f64::<LittleEndian>()?;
        let hazeburnt = reader.read_u8()? != 0;

        let mut item_class = [0; 40];
        for ic in &mut item_class {
            *ic = reader.read_i32::<LittleEndian>()?;
        }
        let mut tree_unlocks = [0; 500];
        for tu in &mut tree_unlocks {
            *tu = reader.read_i32::<LittleEndian>()?;
        }
        let mut class_unlocks = [0; 3];
        for cu in &mut class_unlocks {
            *cu = reader.read_i32::<LittleEndian>()?;
        }

        Ok(Stats {
            level,
            stats,
            xp,
            silver,
            dropped_xp,
            dropped_xp_area,
            dropped_xp_vec: (dx, dy),
            time_played,
            hazeburnt,
            item_class,
            tree_unlocks,
            class_unlocks,
        })
    }

    fn write<W: Write>(&self, writer: &mut W, _version: i32) -> Result<(), SaveError> {
        writer.write_i32::<LittleEndian>(self.level)?;
        for s in &self.stats {
            writer.write_i32::<LittleEndian>(*s)?;
        }
        writer.write_i64::<LittleEndian>(self.xp)?;
        writer.write_i64::<LittleEndian>(self.silver)?;
        writer.write_i64::<LittleEndian>(self.dropped_xp)?;
        writer.write_i32::<LittleEndian>(self.dropped_xp_area)?;
        writer.write_f32::<LittleEndian>(self.dropped_xp_vec.0)?;
        writer.write_f32::<LittleEndian>(self.dropped_xp_vec.1)?;
        writer.write_f64::<LittleEndian>(self.time_played)?;
        writer.write_u8(if self.hazeburnt { 1 } else { 0 })?;
        for ic in &self.item_class {
            writer.write_i32::<LittleEndian>(*ic)?;
        }
        for tu in &self.tree_unlocks {
            writer.write_i32::<LittleEndian>(*tu)?;
        }
        for cu in &self.class_unlocks {
            writer.write_i32::<LittleEndian>(*cu)?;
        }
        Ok(())
    }
}
