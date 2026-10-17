//! Bans, mutes, reports, flags, anticheat. Not implemented in this base project.

/// Always `false`: nothing is ever banned, because nothing is tracked.
pub fn is_banned(_account_id: &str) -> bool {
    false
}
