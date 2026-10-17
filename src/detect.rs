//! Looking at a folder to say what it holds. Not implemented in this base project.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// A kind of Roblox client. Left empty here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RobloxFlavour {}

/// What was found in a folder.
#[derive(Clone, Debug, PartialEq)]
pub struct Detected {
    pub exe: PathBuf,
    pub version: String,
    pub flavour: Option<RobloxFlavour>,
}

/// Always `Err`: nothing is detected in the base project.
pub fn detect(_folder: &Path) -> Result<Detected, String> {
    Err("Not implemented in the RetroVerse base project.".into())
}
