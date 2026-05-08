use crate::utils::{read_string, SaveError};
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek};

#[derive(Debug, Clone)]
pub struct LootField {
    pub id: i32,
    pub data_type: i32,
    pub value: LootFieldValue,
}

#[derive(Debug, Clone)]
pub enum LootFieldValue {
    Float(f32),
    Int(i32),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone)]
pub struct LootDef {
    pub id: i32,
    pub name: String,
    pub title: Vec<String>,
    pub description: Vec<String>,
    pub type_: i32,
    pub sub_type: i32,
    pub cost: f32,
    pub img: i32,
    pub alt_img: i32,
    pub texture: String,
    pub fields: Vec<LootField>,
    pub flags: Vec<i32>,
    pub token_loot: String,
    pub token_cost: i32,
}

pub struct LootCatalog {
    pub loot_defs: Vec<LootDef>,
    pub by_name: HashMap<String, usize>,
    pub black_starstone_index: Option<usize>,
    pub gray_starstone_index: Option<usize>,
}

impl LootCatalog {
    pub fn load_from_bytes(data: &[u8]) -> Result<Self, SaveError> {
        let mut reader = Cursor::new(data);
        let count = reader.read_i32::<LittleEndian>()?;
        #[cfg(debug_assertions)]
        crate::log_loot!("=== Starting to parse {} LootDefs", count);
        let mut defs = Vec::with_capacity(count as usize);
        let mut by_name = HashMap::with_capacity(count as usize);

        for idx in 0..count {
            #[cfg(debug_assertions)]
            crate::log_loot!(
                "\n--- LootDef {} at position {} ---",
                idx,
                reader.stream_position()?
            );
            let def = LootDef::read(&mut reader)?;
            #[cfg(debug_assertions)]
            crate::log_loot!(
                "--- Finished LootDef {} at position {} ---\n",
                idx,
                reader.stream_position()?
            );
            by_name.insert(def.name.clone(), idx as usize);
            defs.push(def);
        }

        let black_pearl_index = defs
            .iter()
            .position(|def| def.name == "black_pearl")
            .or_else(|| {
                defs.iter()
                    .position(|def| def.title.iter().any(|t| t.contains("Black Starstone")))
            });

        let gray_pearl_index = defs
            .iter()
            .position(|def| def.name == "gray_pearl")
            .or_else(|| {
                defs.iter()
                    .position(|def| def.title.iter().any(|t| t.contains("Gray Starstone")))
            });

        Ok(LootCatalog {
            loot_defs: defs,
            by_name,
            black_starstone_index: black_pearl_index,
            gray_starstone_index: gray_pearl_index,
        })
    }
}

impl LootDef {
    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, SaveError> {
        let name = read_string(reader)?;
        #[cfg(debug_assertions)]
        crate::log_loot!("  name: \"{}\" at pos {}", name, reader.stream_position()?);

        let mut title = Vec::with_capacity(20);
        for _i in 0..20 {
            let s = read_string(reader)?;
            #[cfg(debug_assertions)]
            crate::log_loot!(
                "    title[{}]: \"{}\" at pos {}",
                _i,
                s,
                reader.stream_position()?
            );
            title.push(s);
        }

        let mut description = Vec::with_capacity(20);
        for _i in 0..20 {
            let s = read_string(reader)?;
            #[cfg(debug_assertions)]
            crate::log_loot!(
                "    desc[{}]: \"{}\" at pos {}",
                _i,
                s,
                reader.stream_position()?
            );
            description.push(s);
        }

        let type_ = reader.read_i32::<LittleEndian>()?;
        #[cfg(debug_assertions)]
        crate::log_loot!("  type_: {} at pos {}", type_, reader.stream_position()?);
        let sub_type = reader.read_i32::<LittleEndian>()?;
        #[cfg(debug_assertions)]
        crate::log_loot!(
            "  sub_type: {} at pos {}",
            sub_type,
            reader.stream_position()?
        );
        let cost = reader.read_f32::<LittleEndian>()?;
        #[cfg(debug_assertions)]
        crate::log_loot!("  cost: {} at pos {}", cost, reader.stream_position()?);
        let img = reader.read_i32::<LittleEndian>()?;
        #[cfg(debug_assertions)]
        crate::log_loot!("  img: {} at pos {}", img, reader.stream_position()?);
        let alt_img = reader.read_i32::<LittleEndian>()?;
        #[cfg(debug_assertions)]
        crate::log_loot!(
            "  alt_img: {} at pos {}",
            alt_img,
            reader.stream_position()?
        );
        let texture = read_string(reader)?;
        #[cfg(debug_assertions)]
        crate::log_loot!(
            "  texture: \"{}\" at pos {}",
            texture,
            reader.stream_position()?
        );

        let field_count = reader.read_i32::<LittleEndian>()?;
        #[cfg(debug_assertions)]
        crate::log_loot!(
            "  field_count: {} at pos {}",
            field_count,
            reader.stream_position()?
        );
        let mut fields = Vec::with_capacity(field_count as usize);
        for _i in 0..field_count {
            #[cfg(debug_assertions)]
            crate::log_loot!(
                "    reading field {} at pos {}",
                _i,
                reader.stream_position()?
            );
            fields.push(LootField::read(reader)?);
        }

        let flag_count = reader.read_i32::<LittleEndian>()?;
        #[cfg(debug_assertions)]
        crate::log_loot!(
            "  flag_count: {} at pos {}",
            flag_count,
            reader.stream_position()?
        );
        let mut flags = Vec::with_capacity(flag_count as usize);
        for _i in 0..flag_count {
            let flag = reader.read_i32::<LittleEndian>()?;
            #[cfg(debug_assertions)]
            crate::log_loot!(
                "    flag[{}]: {} at pos {}",
                _i,
                flag,
                reader.stream_position()?
            );
            flags.push(flag);
        }

        let token_loot = read_string(reader)?;
        #[cfg(debug_assertions)]
        crate::log_loot!(
            "  token_loot: \"{}\" at pos {}",
            token_loot,
            reader.stream_position()?
        );
        let token_cost = reader.read_i32::<LittleEndian>()?;
        #[cfg(debug_assertions)]
        crate::log_loot!(
            "  token_cost: {} at pos {}",
            token_cost,
            reader.stream_position()?
        );

        Ok(LootDef {
            id: 0,
            name,
            title,
            description,
            type_,
            sub_type,
            cost,
            img,
            alt_img,
            texture,
            fields,
            flags,
            token_loot,
            token_cost,
        })
    }
}

impl LootField {
    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, SaveError> {
        let start_pos = reader.stream_position()?;
        let id = reader.read_i32::<LittleEndian>()?;
        let data_type = reader.read_i32::<LittleEndian>()?;

        let value = match data_type {
            0 => LootFieldValue::Float(reader.read_f32::<LittleEndian>()?),
            2 => LootFieldValue::Int(reader.read_i32::<LittleEndian>()?),
            3 => LootFieldValue::Bool(reader.read_u8()? != 0),
            1 | 4 | 5 | 7 => LootFieldValue::String(read_string(reader)?),
            6 => LootFieldValue::Int(reader.read_i32::<LittleEndian>()?), // magic index is an integer
            _ => {
                return Err(SaveError::InvalidData(format!(
                    "Unknown field type {} at pos {}",
                    data_type, start_pos
                )));
            }
        };

        Ok(LootField {
            id,
            data_type,
            value,
        })
    }
}
