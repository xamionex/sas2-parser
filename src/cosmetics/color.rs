use std::sync::OnceLock;

pub struct CosmeticColor {
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub burnt_r: u8,
    pub burnt_g: u8,
    pub burnt_b: u8,
}

pub struct ColorCatalog;

impl ColorCatalog {
    pub fn get_all() -> &'static [CosmeticColor] {
        static ALL: OnceLock<Vec<CosmeticColor>> = OnceLock::new();
        ALL.get_or_init(|| {
            let entries = [
                (242, 212, 176, "Sunflower Blonde"),
                (242, 226, 201, "Pure Diamond"),
                (186, 141, 112, "Caramel"),
                (241, 212, 182, "Light Ash Blonde"),
                (249, 222, 177, "Light Blonde"),
                (160, 115, 86, "Hot Toffee"),
                (178, 133, 102, "Sparkling Amber"),
                (160, 129, 111, "Havana Brown"),
                (219, 180, 141, "Beeline Honey"),
                (181, 137, 108, "Medium Champagne"),
                (118, 88, 78, "Espresso"),
                (102, 68, 58, "French Roast"),
                (204, 128, 79, "Copper Shimmer"),
                (113, 95, 73, "Light Cool Brown"),
                (96, 74, 60, "Light Brown"),
                (160, 81, 84, "Ruby Fusion"),
                (134, 84, 83, "Crushed Garnet"),
                (111, 63, 77, "Blowout Burgundy"),
                (90, 67, 53, "Chocolate Brown"),
                (88, 65, 47, "Dark Golden Brown"),
                (125, 87, 100, "Chocolate Cherry"),
                (108, 81, 88, "Midnight Ruby"),
                (52, 55, 64, "Leather Black"),
                (220, 161, 129, "Reddish Blonde"),
                (135, 79, 54, "Light Auburn"),
                (255, 0, 0, "Red"),
                (255, 0, 255, "Purple"),
                (255, 150, 255, "Pink"),
                (50, 70, 255, "Blue"),
                (0, 200, 255, "Teal"),
                (0, 255, 0, "Green"),
            ];
            entries
                .into_iter()
                .map(|(r, g, b, name)| {
                    let avg = (r as u16 + g as u16 + b as u16) as f32 / 255.0 / 3.0;
                    let burnt_r = (avg * 0.7 * 255.0) as u8;
                    let burnt_g = (avg * 0.75 * 255.0) as u8;
                    let burnt_b = (avg * 0.8 * 255.0) as u8;
                    CosmeticColor {
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
        Self::get_all().get(idx).map(|c| c.name.as_str())
    }
}
