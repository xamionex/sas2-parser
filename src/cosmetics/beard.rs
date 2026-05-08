use std::sync::OnceLock;

pub struct Beard {
    pub name: String,
    pub img: [Option<String>; 2],
}

pub struct BeardCatalog;

impl BeardCatalog {
    //noinspection SpellCheckingInspection
    pub fn get_all() -> &'static [Beard] {
        static ALL: OnceLock<Vec<Beard>> = OnceLock::new();
        ALL.get_or_init(|| {
            vec![
                Beard {
                    name: "None".to_string(),
                    img: [None, None],
                },
                Beard {
                    name: "Beard".to_string(),
                    img: [
                        Some("beard_beard".to_string()),
                        Some("beard_beard".to_string()),
                    ],
                },
                Beard {
                    name: "Bushy".to_string(),
                    img: [
                        Some("beard_bushy".to_string()),
                        Some("beard_bushy_hood".to_string()),
                    ],
                },
                Beard {
                    name: "Trimmed".to_string(),
                    img: [
                        Some("beard_trimmed".to_string()),
                        Some("beard_trimmed".to_string()),
                    ],
                },
                Beard {
                    name: "Moustache".to_string(),
                    img: [
                        Some("beard_moustache".to_string()),
                        Some("beard_moustache".to_string()),
                    ],
                },
                Beard {
                    name: "Goatee".to_string(),
                    img: [
                        Some("beard_goatee".to_string()),
                        Some("beard_goatee".to_string()),
                    ],
                },
                Beard {
                    name: "Only Goat".to_string(),
                    img: [
                        Some("beard_onlygoat".to_string()),
                        Some("beard_onlygoat".to_string()),
                    ],
                },
                Beard {
                    name: "Chops".to_string(),
                    img: [Some("beard_chops".to_string()), None],
                },
            ]
        })
    }

    pub fn len() -> usize {
        Self::get_all().len()
    }

    pub fn name(idx: usize) -> Option<&'static str> {
        Self::get_all().get(idx).map(|b| b.name.as_str())
    }
}
