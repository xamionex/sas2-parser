use std::sync::OnceLock;

pub struct Sex {
    pub name: String,
    pub path: String,
}

pub struct SexCatalog;

impl SexCatalog {
    pub fn get_all() -> &'static [Sex] {
        static ALL: OnceLock<Vec<Sex>> = OnceLock::new();
        ALL.get_or_init(|| {
            vec![
                Sex {
                    name: "Male".to_string(),
                    path: "male".to_string(),
                },
                Sex {
                    name: "Female".to_string(),
                    path: "female".to_string(),
                },
            ]
        })
    }

    pub fn len() -> usize {
        Self::get_all().len()
    }

    pub fn name(idx: usize) -> Option<&'static str> {
        Self::get_all().get(idx).map(|s| s.name.as_str())
    }
}
