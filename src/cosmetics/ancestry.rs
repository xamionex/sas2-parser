use std::sync::OnceLock;

pub struct Ancestry {
    pub name: String,
    pub path: String,
}

pub struct AncestryCatalog;

impl AncestryCatalog {
    //noinspection SpellCheckingInspection
    pub fn get_all() -> &'static [Ancestry] {
        static ALL: OnceLock<Vec<Ancestry>> = OnceLock::new();
        ALL.get_or_init(|| {
            vec![
                Ancestry {
                    name: "Dusk".to_string(),
                    path: "hero2".to_string(),
                },
                Ancestry {
                    name: "Highlander".to_string(),
                    path: "hero".to_string(),
                },
                Ancestry {
                    name: "Mountain".to_string(),
                    path: "hero6".to_string(),
                },
                Ancestry {
                    name: "Oasis".to_string(),
                    path: "hero7".to_string(),
                },
                Ancestry {
                    name: "Sun".to_string(),
                    path: "hero4".to_string(),
                },
                Ancestry {
                    name: "Wood".to_string(),
                    path: "hero3".to_string(),
                },
                Ancestry {
                    name: "Valley".to_string(),
                    path: "hero5".to_string(),
                },
                Ancestry {
                    name: "Jinderen".to_string(),
                    path: "hero8".to_string(),
                },
                Ancestry {
                    name: "Gulchmire".to_string(),
                    path: "hero9".to_string(),
                },
            ]
        })
    }

    pub fn len() -> usize {
        Self::get_all().len()
    }

    pub fn name(idx: usize) -> Option<&'static str> {
        Self::get_all().get(idx).map(|a| a.name.as_str())
    }
}
