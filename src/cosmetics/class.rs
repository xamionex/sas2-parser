use std::sync::OnceLock;

pub struct CosmeticClass {
    pub name: String,
}

pub struct ClassCatalog;

impl ClassCatalog {
    pub fn get_all() -> &'static [CosmeticClass] {
        static ALL: OnceLock<Vec<CosmeticClass>> = OnceLock::new();
        ALL.get_or_init(|| {
            vec![
                CosmeticClass {
                    name: "Assassin".to_string(),
                },
                CosmeticClass {
                    name: "Cleric".to_string(),
                },
                CosmeticClass {
                    name: "Duelist".to_string(),
                },
                CosmeticClass {
                    name: "Fighter".to_string(),
                },
                CosmeticClass {
                    name: "Highblade".to_string(),
                },
                CosmeticClass {
                    name: "Paladin".to_string(),
                },
                CosmeticClass {
                    name: "Ranger".to_string(),
                },
                CosmeticClass {
                    name: "Sage".to_string(),
                },
            ]
        })
    }

    pub fn len() -> usize {
        Self::get_all().len()
    }

    pub fn name(idx: usize) -> Option<&'static str> {
        Self::get_all().get(idx).map(|c| c.name.as_str())
    }
}
