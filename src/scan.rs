//! Finding builds under a folder. Not implemented in this base project.

use std::path::{Path, PathBuf};

/// Always empty.
pub fn find_builds(_root: &Path, _max_depth: u32) -> Vec<PathBuf> {
    Vec::new()
}
