//! Profiles, equip, favourites, access tokens. Not implemented in this base project.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Profile {
    pub account_id: String,
    pub items: Vec<String>,
}

/// Always `Err`: there is no profile store.
pub fn query_profile(_account_id: &str) -> Result<Profile, String> {
    Err("Not implemented in the RetroVerse base project.".into())
}
