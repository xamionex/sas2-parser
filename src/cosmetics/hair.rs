use std::sync::OnceLock;

pub struct Hair {
    pub name: String,
    pub img: [Option<String>; 3],
}

pub struct HairCatalog;

impl HairCatalog {
    //noinspection SpellCheckingInspection
    pub fn get_all() -> &'static [Hair] {
        static ALL: OnceLock<Vec<Hair>> = OnceLock::new();
        ALL.get_or_init(|| {
            vec![
                Hair {
                    name: "Bald".to_string(),
                    img: [None, None, None],
                },
                Hair {
                    name: "Short".to_string(),
                    img: [Some("hair_short".to_string()), None, None],
                },
                Hair {
                    name: "Shaggy".to_string(),
                    img: [Some("hair_shaggy".to_string()), None, None],
                },
                Hair {
                    name: "Bun".to_string(),
                    img: [Some("hair_bun".to_string()), None, None],
                },
                Hair {
                    name: "Fade".to_string(),
                    img: [Some("hair_fade".to_string()), None, None],
                },
                Hair {
                    name: "Princess".to_string(),
                    img: [Some("hair_princess".to_string()), None, None],
                },
                Hair {
                    name: "Monastic".to_string(),
                    img: [Some("hair_monastic".to_string()), None, None],
                },
                Hair {
                    name: "Mohawk".to_string(),
                    img: [Some("hair_mohawk".to_string()), None, None],
                },
                Hair {
                    name: "Messy".to_string(),
                    img: [Some("hair_messy".to_string()), None, None],
                },
                Hair {
                    name: "Slick".to_string(),
                    img: [Some("hair_slick".to_string()), None, None],
                },
                Hair {
                    name: "Curly".to_string(),
                    img: [Some("hair_curly".to_string()), None, None],
                },
                Hair {
                    name: "Natural".to_string(),
                    img: [Some("hair_natural".to_string()), None, None],
                },
                Hair {
                    name: "Balding".to_string(),
                    img: [Some("hair_balding".to_string()), None, None],
                },
                Hair {
                    name: "Novel".to_string(),
                    img: [Some("hair_novel".to_string()), None, None],
                },
                Hair {
                    name: "Twist Fade".to_string(),
                    img: [Some("hair_twistfade".to_string()), None, None],
                },
                Hair {
                    name: "Short Fade".to_string(),
                    img: [Some("hair_shortfade".to_string()), None, None],
                },
                Hair {
                    name: "Clean Fade".to_string(),
                    img: [Some("hair_cleanfade".to_string()), None, None],
                },
                Hair {
                    name: "Fade Hawk".to_string(),
                    img: [Some("hair_fadehawk".to_string()), None, None],
                },
                Hair {
                    name: "Curly Hawk".to_string(),
                    img: [Some("hair_curlyhawk".to_string()), None, None],
                },
            ]
        })
    }

    /// Returns hair indices sorted alphabetically by name.
    pub fn get_ordered_indices() -> Vec<usize> {
        static ORDERED: OnceLock<Vec<usize>> = OnceLock::new();
        ORDERED
            .get_or_init(|| {
                let mut indexed: Vec<(usize, &str)> = Self::get_all()
                    .iter()
                    .enumerate()
                    .map(|(i, h)| (i, h.name.as_str()))
                    .collect();
                indexed.sort_by(|a, b| a.1.cmp(b.1));
                indexed.into_iter().map(|(i, _)| i).collect()
            })
            .clone()
    }

    pub fn len() -> usize {
        Self::get_all().len()
    }

    pub fn name(idx: usize) -> Option<&'static str> {
        Self::get_all().get(idx).map(|h| h.name.as_str())
    }
}
