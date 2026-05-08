use crate::types::flags::PlayerFlags;

/// Update the `ng_level` field of `PlayerFlags` by scanning the flags.
pub fn update_ng_level(flags: &mut PlayerFlags) {
    flags.ng_level = flags
        .flags
        .iter()
        .filter_map(|f| f.strip_prefix("$&ng_").and_then(|s| s.parse::<i32>().ok()))
        .max()
        .unwrap_or(0);
}

/// Set the NG level by adding/updating the appropriate flag.
pub fn set_ng_level(flags: &mut PlayerFlags, new_level: i32) {
    // Remove any existing NG flag
    flags.flags.retain(|f| !f.starts_with("$&ng_"));
    if new_level > 0 {
        flags.flags.push(format!("$&ng_{}", new_level));
    }
    update_ng_level(flags);
}
