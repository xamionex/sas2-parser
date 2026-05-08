use std::sync::OnceLock;

pub struct EyeColor {
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub burnt_r: u8,
    pub burnt_g: u8,
    pub burnt_b: u8,
}

pub struct EyeCatalog;

impl EyeCatalog {
    pub fn get_all() -> &'static [EyeColor] {
        static ALL: OnceLock<Vec<EyeColor>> = OnceLock::new();
        ALL.get_or_init(|| {
            let entries = [
                (118, 112, 47, "Amber"),
                (0, 144, 255, "Blue"),
                (137, 100, 56, "Brown"),
                (62, 188, 98, "Emerald"),
                (255, 234, 0, "Gold"),
                (0, 228, 255, "Sapphire"),
                (157, 157, 157, "Silver"),
            ];
            entries
                .into_iter()
                .map(|(r, g, b, name)| {
                    let avg = (r as u16 + g as u16 + b as u16) as f32 / 255.0 / 3.0;
                    let burnt_r = (avg * 0.7 * 255.0) as u8;
                    let burnt_g = (avg * 0.75 * 255.0) as u8;
                    let burnt_b = (avg * 0.8 * 255.0) as u8;
                    EyeColor {
                        name: name.to_string(),
                        r,
                        g,
                        b,
                        burnt_r,
                        burnt_g,
                        burnt_b,
                    }
                })
                .collect()
        })
    }

    pub fn len() -> usize {
        Self::get_all().len()
    }

    pub fn name(idx: usize) -> Option<&'static str> {
        Self::get_all().get(idx).map(|e| e.name.as_str())
    }
}
