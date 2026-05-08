use crate::utils::{read_string, SaveError};
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fs;
#[cfg(debug_assertions)]
use std::io::Seek;
use std::io::{Cursor, Read};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum MonsterFieldValue {
    Float(f32),
    Int(i32),
    String(String),
}

#[derive(Debug, Clone)]
pub struct MonsterField {
    pub id: i32,
    pub data_type: i32,
    pub value: MonsterFieldValue,
}

impl MonsterField {
    fn read<R: Read>(reader: &mut R) -> Result<Self, SaveError> {
        let id = reader.read_i32::<LittleEndian>()?;
        let data_type = reader.read_i32::<LittleEndian>()?;
        let value = match data_type {
            0 => MonsterFieldValue::Float(reader.read_f32::<LittleEndian>()?),
            1 | 3 | 4 | 12 => MonsterFieldValue::String(read_string(reader)?),
            2 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 13 => {
                MonsterFieldValue::Int(reader.read_i32::<LittleEndian>()?)
            }
            _ => {
                return Err(SaveError::InvalidData(format!(
                    "Unknown data_type {}",
                    data_type
                )));
            }
        };
        Ok(MonsterField {
            id,
            data_type,
            value,
        })
    }
}

#[derive(Debug, Clone)]
pub struct MonsterDef {
    pub name: String,
    pub titles: Vec<String>,       // exactly 20
    pub descriptions: Vec<String>, // exactly 20
    pub type_: i32,
    pub sub_type: i32,
    pub cost: f32,
    pub img: i32,
    pub alt_img: i32,
    pub texture: String,
    pub def: String,
    pub box_width: i32,
    pub box_height: i32,
    pub box_sub_height: i32,
    pub shadow_width: i32,
    pub shadow_height: i32,
    pub fields: Vec<MonsterField>,
    pub flags: Vec<i32>,
}

impl MonsterDef {
    fn read<R: Read>(reader: &mut R) -> Result<Self, SaveError> {
        let name = read_string(reader)?;
        let mut titles = Vec::with_capacity(20);
        for _ in 0..20 {
            titles.push(read_string(reader)?);
        }
        let mut descriptions = Vec::with_capacity(20);
        for _ in 0..20 {
            descriptions.push(read_string(reader)?);
        }

        let type_ = reader.read_i32::<LittleEndian>()?;
        let sub_type = reader.read_i32::<LittleEndian>()?;
        let cost = reader.read_f32::<LittleEndian>()?;
        let img = reader.read_i32::<LittleEndian>()?;
        let alt_img = reader.read_i32::<LittleEndian>()?;
        let texture = read_string(reader)?;
        let def = read_string(reader)?;
        let box_width = reader.read_i32::<LittleEndian>()?;
        let box_height = reader.read_i32::<LittleEndian>()?;
        let box_sub_height = reader.read_i32::<LittleEndian>()?;
        let shadow_width = reader.read_i32::<LittleEndian>()?;
        let shadow_height = reader.read_i32::<LittleEndian>()?;

        let field_count = reader.read_i32::<LittleEndian>()?;
        let mut fields = Vec::with_capacity(field_count as usize);
        for _ in 0..field_count {
            fields.push(MonsterField::read(reader)?);
        }

        let flag_count = reader.read_i32::<LittleEndian>()?;
        let mut flags = Vec::with_capacity(flag_count as usize);
        for _ in 0..flag_count {
            flags.push(reader.read_i32::<LittleEndian>()?);
        }

        Ok(MonsterDef {
            name,
            titles,
            descriptions,
            type_,
            sub_type,
            cost,
            img,
            alt_img,
            texture,
            def,
            box_width,
            box_height,
            box_sub_height,
            shadow_width,
            shadow_height,
            fields,
            flags,
        })
    }
}

pub struct MonsterCatalog {
    pub monsters: Vec<MonsterDef>,
    pub by_name: HashMap<String, i32>,
}

impl MonsterCatalog {
    pub fn load_from_bytes(data: &[u8]) -> Result<Self, SaveError> {
        let mut reader = Cursor::new(data);
        let count = reader.read_i32::<LittleEndian>()?;
        crate::log_monster!("=== Starting to parse {} Monsters ===", count);

        let mut monsters = Vec::with_capacity(count as usize);
        let mut by_name = HashMap::with_capacity(count as usize);

        for idx in 0..count {
            #[cfg(debug_assertions)]
            crate::log_monster!(
                "\n--- Monster {} at position {} ---",
                idx,
                reader.stream_position()?
            );

            let def = MonsterDef::read(&mut reader)?;

            #[cfg(debug_assertions)]
            crate::log_monster!(
                "  name: \"{}\", type: {}, sub_type: {}, img: {}",
                def.name,
                def.type_,
                def.sub_type,
                def.img
            );
            #[cfg(debug_assertions)]
            crate::log_monster!(
                "  field_count: {}, flag_count: {}",
                def.fields.len(),
                def.flags.len()
            );
            #[cfg(debug_assertions)]
            crate::log_monster!(
                "--- Finished Monster {} at position {} ---\n",
                idx,
                reader.stream_position()?
            );

            by_name.insert(def.name.clone(), idx);
            monsters.push(def);
        }

        Ok(MonsterCatalog { monsters, by_name })
    }

    pub fn load_from_file(path: &Path) -> Result<Self, SaveError> {
        let data = fs::read(path).map_err(|e| SaveError::Io(e.into()))?;
        Self::load_from_bytes(&data)
    }
}
