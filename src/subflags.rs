use crate::utils::{read_string, SaveError};
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::{Cursor, Read};

/// Mirrors Cartographer.TextureSheet.subflags.SubFlagDef
#[derive(Debug, Clone)]
pub struct SubFlagDef {
    pub name: String,
    pub has_vec: bool,
    pub has_rotation: bool,
    pub has_flip: bool,
    pub index_type0: i32,
    pub index_type1: i32,
    pub meta: i32,
    pub options: i32,
    pub item_list: Vec<(String, String)>,
}

impl SubFlagDef {
    fn read<R: Read>(reader: &mut R) -> Result<Self, SaveError> {
        let name = read_string(reader)?;
        let has_vec = reader.read_u8()? != 0;
        let has_rotation = reader.read_u8()? != 0;
        let has_flip = reader.read_u8()? != 0;
        let index_type0 = reader.read_i32::<LittleEndian>()?;
        let index_type1 = reader.read_i32::<LittleEndian>()?;
        let meta = reader.read_i32::<LittleEndian>()?;
        let options = reader.read_i32::<LittleEndian>()?;

        let item_count = reader.read_i32::<LittleEndian>()?;
        let mut item_list = Vec::with_capacity(item_count as usize);
        for _ in 0..item_count {
            let s0 = read_string(reader)?;
            let s1 = read_string(reader)?;
            item_list.push((s0, s1));
        }
        Ok(SubFlagDef { name, has_vec, has_rotation, has_flip, index_type0, index_type1, meta, options, item_list })
    }
}

/// Mirrors the catalog loaded from "Content/gfx/flagdefs.zfd"
pub struct SubFlagDefCatalog {
    pub defs: Vec<SubFlagDef>,
}

impl SubFlagDefCatalog {
    pub fn load_from_path(path: &std::path::Path) -> Result<Self, SaveError> {
        let data = std::fs::read(path).map_err(|e| SaveError::Io(e.into()))?;
        let mut r = Cursor::new(&data[..]);
        let version = r.read_i32::<LittleEndian>()?;
        if version != 100 {
            return Err(SaveError::InvalidData("flagdefs.zfd version != 100".into()));
        }
        let count = r.read_i32::<LittleEndian>()?;
        let mut defs = Vec::with_capacity(count as usize);
        for _ in 0..count {
            defs.push(SubFlagDef::read(&mut r)?);
        }
        Ok(SubFlagDefCatalog { defs })
    }
}