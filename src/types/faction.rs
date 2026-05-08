use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerFaction {
    None,
    Dawnlight,
    Shroud,
    Blueheart,
    Sheriff,
    Oathbound,
    ChaosEater,
}

impl PlayerFaction {
    pub fn get_all() -> &'static [PlayerFaction] {
        static ALL: OnceLock<Vec<PlayerFaction>> = OnceLock::new();
        ALL.get_or_init(|| {
            vec![
                PlayerFaction::None,
                PlayerFaction::Dawnlight,
                PlayerFaction::Shroud,
                PlayerFaction::Blueheart,
                PlayerFaction::Sheriff,
                PlayerFaction::Oathbound,
                PlayerFaction::ChaosEater,
            ]
        })
    }

    pub fn name(&self) -> &'static str {
        match self {
            PlayerFaction::None => "No Faction",
            PlayerFaction::Dawnlight => "Dawnlight Order",
            PlayerFaction::Shroud => "Shrouded Alliance",
            PlayerFaction::Blueheart => "Blueheart Runners",
            PlayerFaction::Sheriff => "Sheriff Inquisitors",
            PlayerFaction::Oathbound => "Oathbound Watchers",
            PlayerFaction::ChaosEater => "Chaos Eaters",
        }
    }

    pub fn flag(&self) -> Option<&'static str> {
        match self {
            PlayerFaction::None => None,
            PlayerFaction::Dawnlight => Some("dawnlight_saved"),
            PlayerFaction::Shroud => Some("shroud_saved"),
            PlayerFaction::Blueheart => Some("blueheart_saved"),
            PlayerFaction::Sheriff => Some("sheriff_saved"),
            PlayerFaction::Oathbound => Some("oath_saved"),
            PlayerFaction::ChaosEater => Some("chaos_saved"),
        }
    }

    /// Determine faction from a list of flags.
    pub fn from_flags(flags: &[String]) -> PlayerFaction {
        for f in flags {
            match f.as_str() {
                "dawnlight_saved" => return PlayerFaction::Dawnlight,
                "shroud_saved" => return PlayerFaction::Shroud,
                "blueheart_saved" => return PlayerFaction::Blueheart,
                "sheriff_saved" => return PlayerFaction::Sheriff,
                "oath_saved" => return PlayerFaction::Oathbound,
                "chaos_saved" => return PlayerFaction::ChaosEater,
                _ => {}
            }
        }
        PlayerFaction::None
    }

    /// Add this faction's flag to the flags list, removing any other faction flags.
    pub fn apply_to_flags(&self, flags: &mut Vec<String>) {
        // Remove all faction flags
        flags.retain(|f| {
            !matches!(
                f.as_str(),
                "dawnlight_saved"
                    | "shroud_saved"
                    | "blueheart_saved"
                    | "sheriff_saved"
                    | "oath_saved"
                    | "chaos_saved"
            )
        });
        // Add the new flag if not None
        if let Some(flag) = self.flag() {
            flags.push(flag.to_string());
        }
    }
}
