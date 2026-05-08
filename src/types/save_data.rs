use crate::types::bestiary::Bestiary;
use crate::types::equipment::Equipment;
use crate::types::flags::PlayerFlags;
use crate::types::serializable::BinarySerializable;
use crate::types::stats::Stats;
use crate::utils::{read_string, write_string, xor_data, SaveError};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use md5;
use std::io::{Cursor, Write};

#[derive(Debug, Clone)]
pub struct SaveData {
    pub version: i32, // original version from file (>100 for salt-guard)
    pub name: String,
    pub stats: Stats,
    pub equipment: Equipment,
    pub flags: PlayerFlags,
    pub bestiary: Bestiary,
    pub cosmetics: [i32; 11],
    pub hash_data: Option<[u8; 16]>,
    pub custom_hash_override: Option<[u8; 16]>,
}

impl SaveData {
    /// Read a save file from raw bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self, SaveError> {
        let mut reader = Cursor::new(data);
        let raw_version = reader.read_i32::<LittleEndian>()?;

        // Determine base version (for format decisions) and XOR key
        let (base_version, xor_key) = if raw_version > 100 {
            // Modded save: subtract 100 to get base version, XOR key always 19
            (raw_version - 100, 19)
        } else {
            // Vanilla: base version = raw_version, XOR key = raw_version for versions 18/19, else 0
            let xor = if raw_version == 18 || raw_version == 19 {
                raw_version
            } else {
                0
            };
            (raw_version, xor)
        };

        let is_mod = raw_version > 100;
        let data_len = data.len() - 4; // total data after version
        let hash_len = if base_version >= 18 { 16 } else { 0 };
        let payload_len = data_len - hash_len;

        let mut data_part = data[4..4 + payload_len].to_vec();
        let hash_part = if hash_len > 0 {
            &data[4 + payload_len..4 + payload_len + hash_len]
        } else {
            &[]
        };

        if xor_key != 0 {
            xor_data(&mut data_part, xor_key);
        }

        let mut payload_reader = Cursor::new(&data_part);

        let name = read_string(&mut payload_reader)?;
        let stats = Stats::read(&mut payload_reader, base_version)?;
        let equipment = Equipment::read(&mut payload_reader, base_version)?;
        let flags = PlayerFlags::read(&mut payload_reader, base_version)?;

        // For versions < 19 (vanilla only), skip 10 ints
        if base_version < 19 && !is_mod {
            for _ in 0..10 {
                payload_reader.read_i32::<LittleEndian>()?;
            }
        }

        let bestiary = Bestiary::read(&mut payload_reader, base_version)?;

        let mut cosmetics = [0; 11];
        for c in &mut cosmetics {
            *c = payload_reader.read_i32::<LittleEndian>()?;
        }

        let pos = payload_reader.position() as usize;
        if pos != payload_len {
            return Err(SaveError::InvalidData(format!(
                "Read {} bytes, expected {}",
                pos, payload_len
            )));
        }

        #[cfg(debug_assertions)]
        {
            println!("========================================");
            println!("        SAVE FILE LOADED SUCCESS        ");
            println!("========================================");
            println!("Name:                 {}", name);
            println!("Raw Version:          {}", raw_version);
            println!("Base Version:         {}", base_version);
            println!("Is Modded Format:     {}", is_mod);
            println!("XOR Key Used:         {}", xor_key);
            println!("Skipped V18 Padding:  {}", base_version < 19 && !is_mod);
            println!("----------------------------------------");
            println!("Inventory Count:      {}", equipment.inventory_items.len());
            println!("Active Flags Count:   {}", flags.flags.len());
            println!("Bestiary Count:       {}", bestiary.beasts.len());
            println!("Payload Size (bytes): {}", payload_len);
            println!("Final Read Cursor:    {}", pos);
            println!("========================================");
        }

        // Verify hash if present (base_version >= 18)
        if base_version >= 18 {
            let mut stored_hash = [0; 16];
            stored_hash.copy_from_slice(hash_part);
            let computed_hash = md5::compute(&data_part);
            if computed_hash.0 != stored_hash {
                return Err(SaveError::HashMismatch);
            }
        }

        Ok(SaveData {
            version: raw_version,
            name,
            stats,
            equipment,
            flags,
            bestiary,
            cosmetics,
            hash_data: if base_version >= 18 {
                let mut stored_hash = [0; 16];
                stored_hash.copy_from_slice(hash_part);
                Some(stored_hash)
            } else {
                None
            },
            custom_hash_override: None,
        })
    }

    /// Internal helper to write save data with an optional version override.
    fn to_bytes_internal(&self, version_override: Option<i32>) -> Result<Vec<u8>, SaveError> {
        let raw_version = version_override.unwrap_or(self.version);

        let (base_version, xor_key) = if raw_version > 100 {
            (raw_version - 100, 19)
        } else {
            (
                raw_version,
                if raw_version == 18 || raw_version == 19 {
                    raw_version
                } else {
                    0
                },
            )
        };

        let is_mod = raw_version > 100;
        let mut data_part = Vec::new();

        write_string(&mut data_part, &self.name)?;
        self.stats.write(&mut data_part, base_version)?;
        self.equipment.write(&mut data_part, base_version)?;
        self.flags.write(&mut data_part, base_version)?;

        // Padding for vanilla versions < 19 (40 bytes)
        if base_version < 19 && !is_mod {
            for _ in 0..10 {
                data_part.write_i32::<LittleEndian>(0)?;
            }
        }

        self.bestiary.write(&mut data_part, base_version)?;
        for c in &self.cosmetics {
            data_part.write_i32::<LittleEndian>(*c)?;
        }

        // Compute hash on the unencrypted data_part
        let hash_bytes = if let Some(custom) = self.custom_hash_override {
            custom
        } else if base_version >= 18 {
            md5::compute(&data_part).0
        } else {
            [0u8; 16]
        };

        if xor_key != 0 {
            xor_data(&mut data_part, xor_key);
        }

        let mut out = Vec::new();
        out.write_i32::<LittleEndian>(raw_version)?;
        out.write_all(&data_part)?;
        if base_version >= 18 {
            out.write_all(&hash_bytes)?;
        }

        Ok(out)
    }

    /// Write the save data to raw bytes using the existing version.
    pub fn to_bytes(&self) -> Result<Vec<u8>, SaveError> {
        self.to_bytes_internal(None)
    }

    /// Convert the save to a vanilla format with the given target version (≤100).
    pub fn to_vanilla_bytes(&self, target_version: i32) -> Result<Vec<u8>, SaveError> {
        if target_version > 100 {
            return Err(SaveError::InvalidData(
                "Vanilla version must be ≤ 100".to_string(),
            ));
        }
        self.to_bytes_internal(Some(target_version))
    }
}
