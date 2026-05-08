use image::RgbaImage;
use serde::Serialize;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use xnb::{BmFont, Effect, MaybeCompressedXNB, SoundEffect, SpriteFont, Texture2d, WindowSize};

/// All asset types we can export.
pub enum XnbAsset {
    Texture(RgbaImage),
    SpriteFont(SpriteFont),
    Effect(Effect),
    SoundEffect(SoundEffect),
    BmFont(String), // XML string
    Unknown(Vec<u8>),
}

/// Extracts the primary asset from the XNB container, handling both compressed and uncompressed variants.
fn unpack_primary<T: xnb::Parse>(xnb: MaybeCompressedXNB) -> Result<T, String> {
    match xnb {
        MaybeCompressedXNB::Uncompressed(xnb) => xnb
            .xnb()
            .map_err(|e| format!("Uncompressed error: {:?}", e))
            .map(|x| x.primary),
        MaybeCompressedXNB::Compressed(xnb) => xnb
            .xnb(WindowSize::KB64)
            .map_err(|e| format!("Compressed error: {:?}", e))
            .map(|x| x.primary),
    }
}

/// Helper to parse the XNB container from a byte buffer.
fn parse_primary_from_buffer<T: xnb::Parse>(data: &[u8]) -> Result<T, String> {
    let mut cursor = Cursor::new(data);
    let xnb = MaybeCompressedXNB::from_buffer(&mut cursor)
        .map_err(|e| format!("XNB parse error: {:?}", e))?;
    unpack_primary(xnb)
}

/// Load an XNB file and return the parsed asset.
pub fn load_asset_from_xnb(path: &str) -> Result<XnbAsset, String> {
    // Read the file once into memory
    let data = fs::read(path).map_err(|e| e.to_string())?;

    // Check the type name to decide which loader to use
    let mut cursor = Cursor::new(&data);
    let type_name = xnb::get_asset_type_name(&mut cursor)
        .map_err(|e| format!("Failed to get asset type: {:?}", e))?;

    #[cfg(debug_assertions)]
    println!("Asset type: {}", type_name);

    // Handle SpriteFont
    if type_name.starts_with("Microsoft.Xna.Framework.Content.SpriteFontReader") {
        return load_spritefont_from_xnb(&data).map(XnbAsset::SpriteFont);
    }

    match type_name.as_str() {
        "Microsoft.Xna.Framework.Content.Texture2DReader" => {
            load_texture_from_xnb(&data).map(XnbAsset::Texture)
        }
        "Microsoft.Xna.Framework.Content.EffectReader" => {
            load_effect_from_xnb(&data).map(XnbAsset::Effect)
        }
        "Microsoft.Xna.Framework.Content.SoundEffectReader" => {
            load_sound_effect_from_xnb(&data).map(XnbAsset::SoundEffect)
        }
        "BmFont.XmlSourceReader" => load_bitmap_font_from_xnb(&data).map(XnbAsset::BmFont),
        _ => Ok(XnbAsset::Unknown(data)),
    }
}

pub fn load_texture_from_xnb(data: &[u8]) -> Result<RgbaImage, String> {
    let texture: Texture2d = parse_primary_from_buffer(data)?;

    if !matches!(texture.format, xnb::SurfaceFormat::Color) {
        return Err(format!("Unsupported texture format: {:?}", texture.format));
    }

    let mip_data = &texture.mip_data[0];
    RgbaImage::from_raw(
        texture.width as u32,
        texture.height as u32,
        mip_data.clone(),
    )
    .ok_or_else(|| "Failed to create RgbaImage".to_string())
}

pub fn load_spritefont_from_xnb(data: &[u8]) -> Result<SpriteFont, String> {
    parse_primary_from_buffer(data)
}

pub fn load_effect_from_xnb(data: &[u8]) -> Result<Effect, String> {
    parse_primary_from_buffer(data)
}

pub fn load_sound_effect_from_xnb(data: &[u8]) -> Result<SoundEffect, String> {
    parse_primary_from_buffer(data)
}

pub fn load_bitmap_font_from_xnb(data: &[u8]) -> Result<String, String> {
    let bitmap_font: BmFont = parse_primary_from_buffer(data)?;
    Ok(bitmap_font.xml)
}

pub fn load_texture_from_path(path: &str) -> Result<RgbaImage, String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;
    load_texture_from_xnb(&data)
}

pub fn load_spritefont_from_path(path: &str) -> Result<SpriteFont, String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;
    load_spritefont_from_xnb(&data)
}

/// Determine the appropriate file extension for an XnbAsset.
pub fn asset_extension(asset: &XnbAsset) -> &'static str {
    match asset {
        XnbAsset::Texture(_) => "png",
        XnbAsset::Effect(_) => "cso",
        XnbAsset::SoundEffect(_) => "wav",
        XnbAsset::BmFont(_) => "xml",
        XnbAsset::SpriteFont(_) => "json",
        XnbAsset::Unknown(_) => "bin",
    }
}

#[derive(Serialize)]
struct SerializableSpriteFont {
    texture: SerializableTexture,
    glyphs: Vec<(i32, i32, i32, i32)>,
    cropping: Vec<(i32, i32, i32, i32)>,
    char_map: Vec<char>,
    v_spacing: i32,
    h_spacing: f32,
    kerning: Vec<(f32, f32, f32)>,
    default: Option<char>,
}

#[derive(Serialize)]
struct SerializableTexture {
    format: String,
    width: usize,
    height: usize,
}

/// Write an XnbAsset to a file at the given path.
pub fn export_asset_to_file(asset: XnbAsset, output_path: &Path) -> Result<(), String> {
    match asset {
        XnbAsset::Texture(img) => img
            .save_with_format(output_path, image::ImageFormat::Png)
            .map_err(|e| format!("Failed to save PNG: {}", e)),
        XnbAsset::Effect(effect) => fs::write(output_path, effect.data).map_err(|e| e.to_string()),
        XnbAsset::SoundEffect(sound) => {
            let wav = sound.to_wav();
            fs::write(output_path, wav).map_err(|e| e.to_string())
        }
        XnbAsset::BmFont(xml) => fs::write(output_path, xml).map_err(|e| e.to_string()),
        XnbAsset::SpriteFont(font) => {
            let serializable = SerializableSpriteFont {
                texture: SerializableTexture {
                    format: format!("{:?}", font.texture.format),
                    width: font.texture.width,
                    height: font.texture.height,
                },
                glyphs: font.glyphs.iter().map(|r| (r.x, r.y, r.w, r.h)).collect(),
                cropping: font.cropping.iter().map(|r| (r.x, r.y, r.w, r.h)).collect(),
                char_map: font.char_map.clone(),
                v_spacing: font.v_spacing,
                h_spacing: font.h_spacing,
                kerning: font.kerning.iter().map(|v| (v.0, v.1, v.2)).collect(),
                default: font.default,
            };

            let json = serde_json::to_string_pretty(&serializable)
                .map_err(|e| format!("Failed to serialize SpriteFont: {}", e))?;
            fs::write(output_path, json)
                .map_err(|e| format!("Failed to write font JSON: {}", e))?;

            // Export internal font texture
            let texture = &font.texture;
            if !matches!(texture.format, xnb::SurfaceFormat::Color) {
                return Err(format!("Unsupported texture format: {:?}", texture.format));
            }

            let img = RgbaImage::from_raw(
                texture.width as u32,
                texture.height as u32,
                texture.mip_data[0].clone(),
            )
            .ok_or("Failed to create RGBA image from font texture")?;

            let texture_path = output_path.with_extension("png");
            img.save(&texture_path)
                .map_err(|e| format!("Failed to save font texture: {}", e))?;

            Ok(())
        }
        XnbAsset::Unknown(data) => fs::write(output_path, data).map_err(|e| e.to_string()),
    }
}
