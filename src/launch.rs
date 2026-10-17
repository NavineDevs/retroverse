//! Starting a build. Not implemented in this base project.

use std::process::Child;

use crate::library::Build;

/// Always `Err`: nothing is started.
pub fn launch(_build: &Build) -> Result<Child, String> {
    Err("Not implemented in the RetroVerse base project.".into())
}
