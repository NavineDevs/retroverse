//! The games RetroVerse can list. Naming versions is not implemented in this base project.

use serde::{Deserialize, Serialize};

/// A game a build belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Game {
    Fortnite,
    Roblox,
}

impl Game {
    pub fn name(self) -> &'static str {
        match self {
            Self::Fortnite => "Fortnite",
            Self::Roblox => "Roblox",
        }
    }
}
