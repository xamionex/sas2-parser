use crate::utils::{read_string, SaveError};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor};

/// A single sprite part, as stored in the '.zsx' file (Part.Read in C#).
#[derive(Debug, Clone)]
pub struct Part {
    /// Tile index on the sprite sheet (≥ 0 = valid, < 0 = hidden).
    pub idx: i32,
    /// Position of the part's centre in local (character) space.
    pub location: (f32, f32),
    /// Rotation in radians.
    pub rotation: f32,
    /// Scale (x, y).
    pub scaling: (f32, f32),
    /// Horizontal flip flag (0 = normal, non-zero = flipped).
    pub flip: i32,
    /// Parent part index (-1 = root).
    pub parent: i32,
    /// Offset from parent in parent-local space (only meaningful when parent ≥ 0).
    pub parent_loc_offset: (f32, f32),
    /// Rotation offset relative to parent (only meaningful when parent ≥ 0).
    pub parent_rotation_offset: f32,
}

/// One animation frame, containing up to 32 parts.
#[derive(Debug, Clone)]
pub struct Frame {
    pub parts: Vec<Part>,
}

/// A single keyframe inside an animation sequence.
#[derive(Debug, Clone)]
pub struct KeyFrame {
    /// Index into the 'CharDef::frames' list that this keyframe displays.
    pub frame_ref: i32,
}

/// One named animation (a sequence of keyframes).
#[derive(Debug, Clone)]
pub struct Animation {
    pub name: String,
    pub key_frames: Vec<KeyFrame>,
}

/// Character definition loaded from a '.zsx' file.
///
/// Mirrors 'Skellingtons.character.def.CharDef' (ReadShort path).
#[derive(Debug, Clone)]
pub struct CharDef {
    pub name: String,
    pub tex_name: String,
    pub animations: Vec<Animation>,
    /// Frames are stored in the order they appear in the file.
    /// A keyframe's 'frame_ref' directly indexes into this vector.
    pub frames: Vec<Frame>,
}

impl CharDef {
    /// Parse raw '.zsx' bytes.
    ///
    /// Format (C# 'CharDef.ReadShort'):
    ///
    /// string   path
    /// string   texName
    /// i32      specTex
    ///
    /// i32      anim_count
    /// for each animation:
    ///   string   name               // empty ⟹ skip rest of this entry
    ///   i32      keyframe_count
    ///   for each keyframe:
    ///     i32    frameRef
    ///     i32    duration
    ///     bool   lerp               // 1 byte
    ///     u8     script_count
    ///     for each script: string
    ///
    /// i32      frame_count
    /// for each k in 0..frame_count:
    ///   bool   exists              // 1 byte, false ⟹ frame k is absent
    ///   if exists:
    ///     i32  parts_count
    ///     for each part (Part.Read):
    ///       i32  idx
    ///       f32  location.X
    ///       f32  location.Y
    ///       f32  rotation
    ///       f32  scaling.X
    ///       f32  scaling.Y
    ///       i32  flip
    ///       i32  parent
    ///       if parent > -1:
    ///         f32  parentLocOffset.X
    ///         f32  parentLocOffset.Y
    ///         f32  parentRotationOffset
    ///
    pub fn load_from_bytes(data: &[u8], name: String) -> Result<Self, SaveError> {
        let mut r = Cursor::new(data);

        // header
        let _path     = read_string(&mut r)?;
        let tex_name = read_string(&mut r)?;
        let _spec_tex = r.read_i32::<LittleEndian>()?;

        // animations
        let anim_count = r.read_i32::<LittleEndian>()?;
        if anim_count < 0 || anim_count > 2_000 {
            return Err(SaveError::InvalidData(
                format!("Implausible animation count {}", anim_count),
            ));
        }
        let mut animations = Vec::with_capacity(anim_count as usize);

        for _ in 0..anim_count {
            let anim_name = read_string(&mut r)?;
            if anim_name.is_empty() {
                // Game code writes the entry but skips reading the rest when the name is empty, so we do the same.
                continue;
            }

            let kf_count = r.read_i32::<LittleEndian>()?;
            if kf_count < 0 || kf_count > 10_000 {
                return Err(SaveError::InvalidData(
                    format!("Implausible keyframe count {}", kf_count),
                ));
            }
            let mut key_frames = Vec::with_capacity(kf_count as usize);

            for _ in 0..kf_count {
                let frame_ref = r.read_i32::<LittleEndian>()?;
                let _duration = r.read_i32::<LittleEndian>()?;
                let _lerp     = r.read_u8()?;

                let script_count = r.read_u8()?;
                for _ in 0..script_count {
                    let _script = read_string(&mut r)?;
                }

                key_frames.push(KeyFrame { frame_ref });
            }

            animations.push(Animation { name: anim_name, key_frames });
        }

        // frames
        let frame_count = r.read_i32::<LittleEndian>()?;
        if frame_count < 0 || frame_count > 20_000 {
            return Err(SaveError::InvalidData(
                format!("Implausible frame count {}", frame_count),
            ));
        }
        let mut frames = Vec::new();

        for _k in 0..frame_count {
            let exists = r.read_u8()? != 0;
            if !exists {
                // Absent frame, skip all data (the game simply doesn't add it to the list).
                continue;
            }

            let parts_count = r.read_i32::<LittleEndian>()?;
            if parts_count < 0 || parts_count > 32 {
                return Err(SaveError::InvalidData(
                    format!("Implausible parts count {} at frame index {}", parts_count, _k),
                ));
            }

            let mut parts = Vec::with_capacity(parts_count as usize);
            for _ in 0..parts_count {
                let idx       = r.read_i32::<LittleEndian>()?;
                let loc_x     = r.read_f32::<LittleEndian>()?;
                let loc_y     = r.read_f32::<LittleEndian>()?;
                let rotation  = r.read_f32::<LittleEndian>()?;
                let scale_x   = r.read_f32::<LittleEndian>()?;
                let scale_y   = r.read_f32::<LittleEndian>()?;
                let flip      = r.read_i32::<LittleEndian>()?;
                let parent    = r.read_i32::<LittleEndian>()?;

                let (parent_loc_offset, parent_rotation_offset) = if parent > -1 {
                    let ox  = r.read_f32::<LittleEndian>()?;
                    let oy  = r.read_f32::<LittleEndian>()?;
                    let rot = r.read_f32::<LittleEndian>()?;
                    ((ox, oy), rot)
                } else {
                    ((0.0, 0.0), 0.0)
                };

                parts.push(Part {
                    idx,
                    location: (loc_x, loc_y),
                    rotation,
                    scaling: (scale_x, scale_y),
                    flip,
                    parent,
                    parent_loc_offset,
                    parent_rotation_offset,
                });
            }

            frames.push(Frame { parts });
        }

        Ok(CharDef { name, tex_name, animations, frames })
    }

    /// Open and parse a '.zsx' file directly from disk.
    pub fn load_from_path(path: &std::path::Path) -> Result<Self, SaveError> {
        let name = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let data = std::fs::read(path).map_err(|e| SaveError::Io(e.into()))?;
        Self::load_from_bytes(&data, name)
    }

    /// Return the frame for the idle animation, or 'None' if unavailable.
    ///
    /// Looks for an animation named "idle", falls back to animation[0],
    /// and returns the frame referenced by its first keyframe.
    pub fn idle_frame(&self) -> Option<&Frame> {
        let anim = self.animations.iter()
            .find(|a| a.name == "idle")
            .or_else(|| self.animations.first())?;

        let frame_ref = anim.key_frames.first()?.frame_ref;
        let idx = frame_ref as usize;

        // If the index is out of bounds (shouldn't happen, but be safe), fall back to the first frame.
        if idx >= self.frames.len() {
            eprintln!("Idle frame_ref {} out of {} frames, using first frame", idx, self.frames.len());
            self.frames.first()
        } else {
            Some(&self.frames[idx])
        }
    }
}