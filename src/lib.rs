use std::sync::atomic::{AtomicBool, Ordering};

static LOOT_LOGGING_ENABLED: AtomicBool = AtomicBool::new(cfg!(debug_assertions));
static MONSTER_LOGGING_ENABLED: AtomicBool = AtomicBool::new(cfg!(debug_assertions));

pub fn set_loot_logging_enabled(enabled: bool) {
    LOOT_LOGGING_ENABLED.store(enabled, Ordering::Relaxed);
}

pub fn set_monster_logging_enabled(enabled: bool) {
    MONSTER_LOGGING_ENABLED.store(enabled, Ordering::Relaxed);
}

// Helper for conditional logging
#[macro_export]
macro_rules! log_loot {
    ($($arg:tt)*) => {
        if $crate::LOOT_LOGGING_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            eprintln!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! log_monster {
    ($($arg:tt)*) => {
        if $crate::MONSTER_LOGGING_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            eprintln!($($arg)*);
        }
    };
}

pub mod cosmetics;
pub mod loot_catalog;
pub mod loot_names;
pub mod monster_catalog;
pub mod skilltree;
mod utils;
pub mod xnb_loader;

pub mod types;
pub mod char_def;
pub mod subflags;
pub mod xtexture;

pub use types::{Bestiary, BestiaryBeast, Equipment, Item, PlayerFlags, SaveData, Stats};
