//! Chat, friends, parties, presence. Not implemented in this base project.

/// Always `Err`: there is no socket to accept into.
pub fn accept_connection() -> Result<(), String> {
    Err("Not implemented in the RetroVerse base project.".into())
}
