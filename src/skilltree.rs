use crate::utils::read_string;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct SkillNode {
    pub id: usize,
    pub name: String,
    pub titles: Vec<String>,            // 13 languages
    pub descriptions: Vec<String>,      // 13 languages
    pub base_descriptions: Vec<String>, // 13 languages
    pub node_type: i32,
    pub value: i32,
    pub cost: i32,
    pub parents: [i32; 2],
    pub loc_x: f32,
    pub loc_y: f32,
}

impl SkillNode {
    pub fn max_unlock(&self) -> i32 {
        if self.cost > 1 {
            1
        } else if self.node_type <= 8 {
            5
        } else {
            1
        }
    }

    pub fn stat_name(&self) -> Option<&'static str> {
        match self.node_type {
            0 => Some("Strength"),
            1 => Some("Dexterity"),
            2 => Some("Vitality"),
            3 => Some("Will"),
            4 => Some("Endurance"),
            5 => Some("Arcana"),
            6 => Some("Conviction"),
            7 => Some("Resolve"),
            8 => Some("Luck"),
            _ => None,
        }
    }
}

pub struct SkillTreeCatalog {
    pub nodes: Vec<SkillNode>,
}

impl SkillTreeCatalog {
    pub fn load_from_bytes(data: &[u8]) -> Result<Self, String> {
        let mut reader = Cursor::new(data);
        let count = reader
            .read_i32::<LittleEndian>()
            .map_err(|e| e.to_string())?;
        let mut nodes = Vec::with_capacity(count as usize);

        for id in 0..count {
            let name = read_string(&mut reader).map_err(|e| e.to_string())?;

            let mut titles = Vec::with_capacity(13);
            for _ in 0..13 {
                titles.push(read_string(&mut reader).map_err(|e| e.to_string())?);
            }

            let mut descriptions = Vec::with_capacity(13);
            for _ in 0..13 {
                descriptions.push(read_string(&mut reader).map_err(|e| e.to_string())?);
            }

            let mut base_descriptions = Vec::with_capacity(13);
            for _ in 0..13 {
                base_descriptions.push(read_string(&mut reader).map_err(|e| e.to_string())?);
            }

            let node_type = reader
                .read_i32::<LittleEndian>()
                .map_err(|e| e.to_string())?;
            let value = reader
                .read_i32::<LittleEndian>()
                .map_err(|e| e.to_string())?;
            let cost = reader
                .read_i32::<LittleEndian>()
                .map_err(|e| e.to_string())?;

            let mut parents = [0; 2];
            for i in 0..2 {
                parents[i] = reader
                    .read_i32::<LittleEndian>()
                    .map_err(|e| e.to_string())?;
            }

            let loc_x = reader
                .read_f32::<LittleEndian>()
                .map_err(|e| e.to_string())?;
            let loc_y = reader
                .read_f32::<LittleEndian>()
                .map_err(|e| e.to_string())?;

            nodes.push(SkillNode {
                id: id as usize,
                name,
                titles,
                descriptions,
                base_descriptions,
                node_type,
                value,
                cost,
                parents,
                loc_x,
                loc_y,
            });
        }

        Ok(SkillTreeCatalog { nodes })
    }

    pub fn load_from_path(path: &std::path::Path) -> Result<Self, String> {
        let data = std::fs::read(path).map_err(|e| e.to_string())?;
        Self::load_from_bytes(&data)
    }
}

// Icon mapping from node_type to texture atlas index (from SkillNode.skillImg in C#)
pub const SKILL_IMG: [i32; 32] = [
    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 10, 12, 11, 40, 43, 41, 39, 14, 46, 42, 13, 15, 45, 47,
    44, 37, 34, 141, 157, 205, 173, 189,
];
