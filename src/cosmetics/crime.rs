use std::sync::OnceLock;

pub struct Crime {
    pub name: String,
}

pub struct CrimeCatalog;

impl CrimeCatalog {
    pub fn get_all() -> &'static [Crime] {
        static ALL: OnceLock<Vec<Crime>> = OnceLock::new();
        ALL.get_or_init(|| {
            // Order and names match CrimeCatalog.Init() and the constants in the game.
            // These are the English names; the actual localized names are in LocStrings.
            // For the editor, we use the English names as they are commonly known.
            vec![
                Crime {
                    name: "Alchemy".to_string(),
                },
                Crime {
                    name: "Arson".to_string(),
                },
                Crime {
                    name: "Blasphemy".to_string(),
                },
                Crime {
                    name: "Brigandry".to_string(),
                },
                Crime {
                    name: "Drunkenness".to_string(),
                },
                Crime {
                    name: "Forgery".to_string(),
                },
                Crime {
                    name: "Heresy".to_string(),
                },
                Crime {
                    name: "Lasciviousness".to_string(),
                },
                Crime {
                    name: "Smuggling".to_string(),
                },
                Crime {
                    name: "Sumptuousness".to_string(),
                },
                Crime {
                    name: "Usury".to_string(),
                },
                Crime {
                    name: "Vagrancy".to_string(),
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
