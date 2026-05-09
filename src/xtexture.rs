use crate::subflags::SubFlagDefCatalog;
use crate::utils::{read_string, read_string_lossy, SaveError};
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::Cursor;

/// A sprite cell with its source rectangle and origin, taken from the XTexture metadata.
#[derive(Debug, Clone)]
pub struct XSprite {
    pub src_rect: (i32, i32, i32, i32), // x, y, width, height
    pub origin: (f32, f32),             // pivot point in source-space
    pub flags: i32,                     // clothing index (unused for assembly)
}

/// Lightweight version of the XTexture metadata, only the cell array.
#[derive(Debug, Clone)]
pub struct XTextureMeta {
    pub cells: Vec<Option<XSprite>>,
}

impl XTextureMeta {
    /// Load metadata from a standalone '.zsx' texture file (e.g. gfx/arenacheck.zsx).
    /// Uses strict UTF-8 because standalone files are written by the editor/tools.
    pub fn load_from_bytes(data: &[u8], flag_defs: &SubFlagDefCatalog) -> Result<Self, SaveError> {
        let mut r = Cursor::new(data);
        Self::read_from_reader(&mut r, flag_defs)
    }

    /// Convenience: open a file and call 'load_from_bytes'.
    pub fn load_from_path(
        path: &std::path::Path,
        flag_defs: &SubFlagDefCatalog,
    ) -> Result<Self, SaveError> {
        let data = std::fs::read(path).map_err(|e| SaveError::Io(e.into()))?;
        Self::load_from_bytes(&data, flag_defs)
    }

    /// Load **all** texture metadata from a 'master.zcm' bundle.
    /// Uses **lossy** string conversion because some names contain non-UTF-8 bytes.
    pub fn load_all_from_master_bytes(
        data: &[u8],
        flag_defs: &SubFlagDefCatalog,
    ) -> Result<HashMap<String, XTextureMeta>, SaveError> {
        let mut r = Cursor::new(data);
        let count = r.read_i32::<LittleEndian>()?;
        let mut map = HashMap::with_capacity(count as usize);

        for _ in 0..count {
            // Read the texture name, lossy, because the game stores ANSI names here.
            let name = read_string_lossy(&mut r)?;
            let meta = Self::read_from_reader(&mut r, flag_defs)?;
            map.insert(name, meta);
        }

        Ok(map)
    }

    /// Convenience: open a 'master.zcm' file and parse it.
    pub fn load_all_from_master_path(
        path: &std::path::Path,
        flag_defs: &SubFlagDefCatalog,
    ) -> Result<HashMap<String, XTextureMeta>, SaveError> {
        let data = std::fs::read(path).map_err(|e| SaveError::Io(e.into()))?;
        Self::load_all_from_master_bytes(&data, flag_defs)
    }

    /// Reads the XTexture data (type + cells) from the current cursor position.
    fn read_from_reader(
        r: &mut Cursor<&[u8]>,
        flag_defs: &SubFlagDefCatalog,
    ) -> Result<Self, SaveError> {
        let texture_type = r.read_i32::<LittleEndian>()?;
        let cell_count = r.read_i32::<LittleEndian>()?;
        let mut cells = Vec::with_capacity(cell_count as usize);

        for _ in 0..cell_count {
            let exists = r.read_u8()? != 0;
            if !exists {
                cells.push(None);
                continue;
            }

            // Read an XSprite
            let _name = read_string(r)?;                // sprite name, strict UTF-8 is fine
            let src_x = r.read_i32::<LittleEndian>()?;
            let src_y = r.read_i32::<LittleEndian>()?;
            let src_w = r.read_i32::<LittleEndian>()?;
            let src_h = r.read_i32::<LittleEndian>()?;
            let origin_x = r.read_f32::<LittleEndian>()?;
            let origin_y = r.read_f32::<LittleEndian>()?;

            let subflag_count = r.read_i32::<LittleEndian>()?;
            for _ in 0..subflag_count {
                skip_subflag(r, flag_defs)?;
            }

            // Read type-specific per-cell fields.
            // For monster textures (type 0) there are none, but we still have to
            // skip them when the texture is of another type, otherwise the cursor
            // will be misaligned for the next entry.
            let flags = match texture_type {
                1 => {
                    // TYPE_CLOTHES: char_ref (i32), flags (i32)
                    let _char_ref = r.read_i32::<LittleEndian>()?;
                    r.read_i32::<LittleEndian>()?
                }
                2 | 4 => {
                    // TYPE_MAP / TYPE_CHAR: flags (i32)
                    r.read_i32::<LittleEndian>()?
                }
                _ => 0, // TYPE_NORMAL / TYPE_CHARACTER, no extra fields
            };

            cells.push(Some(XSprite {
                src_rect: (src_x, src_y, src_w, src_h),
                origin: (origin_x, origin_y),
                flags,
            }));
        }

        Ok(XTextureMeta { cells })
    }
}

/// Skips one XSpriteSubFlag in the stream, advancing the cursor correctly.
fn skip_subflag(
    r: &mut Cursor<&[u8]>,
    catalog: &SubFlagDefCatalog,
) -> Result<(), SaveError> {
    let def_idx = r.read_i32::<LittleEndian>()? as usize;
    if def_idx >= catalog.defs.len() {
        return Err(SaveError::InvalidData(format!(
            "Flag def index {} out of range (max {})",
            def_idx,
            catalog.defs.len()
        )));
    }
    let def = &catalog.defs[def_idx];

    if def.has_vec {
        let _ = r.read_f32::<LittleEndian>()?;
        let _ = r.read_f32::<LittleEndian>()?;
    }
    if def.has_rotation {
        let _ = r.read_f32::<LittleEndian>()?;
    }
    if def.has_flip {
        let _ = r.read_u8()?;
    }
    if def.index_type0 > 0 {
        let _ = r.read_i32::<LittleEndian>()?;
    }
    if def.index_type1 > 0 {
        let _ = r.read_i32::<LittleEndian>()?;
    }
    if def.options > 0 {
        let _ = r.read_i32::<LittleEndian>()?;
    }
    Ok(())
}