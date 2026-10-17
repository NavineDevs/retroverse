//! The saved list of builds.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::catalog::Game;
use crate::detect::RobloxFlavour;

/// One build the library knows about.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Build {
    pub game: Game,
    /// The folder the build was found in.
    pub folder: PathBuf,
    /// The executable that starts it.
    pub exe: PathBuf,
    #[serde(default)]
    pub flavour: Option<RobloxFlavour>,
    /// What was read from the build: a version, a build number, or an era tag.
    pub version: String,
    /// The name it is shown under.
    pub label: String,
    /// Extra command-line arguments, written the way a shell would take them.
    #[serde(default)]
    pub extra_args: String,
    /// A program to start the build through, such as `wine` or `gamemoderun wine`. Empty to
    /// start it directly.
    #[serde(default)]
    pub wrapper: String,
}

impl Build {
    /// Works out what is in `folder` and makes a build of it.
    pub fn from_folder(_folder: &Path) -> Result<Self, String> {
        Err("Not implemented in the RetroVerse base project.".into())
    }
}

/// The library: every build, in the order they were added.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Library {
    #[serde(default)]
    pub builds: Vec<Build>,
}

/// Where the library is kept: the platform's per-user data folder.
pub fn default_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("RetroVerse")
        .join("library.json")
}

impl Library {
    /// The saved library, or an empty one. A file that cannot be read or understood is treated as
    /// no library rather than as an error: the window should open, and the worst case is an
    /// empty list that the next scan fills again.
    pub fn load() -> Self {
        Self::load_from(&default_path())
    }

    pub fn load_from(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        self.save_to(&default_path())
    }

    /// Written to a temporary file and renamed into place, so a crash half way through leaves the
    /// old library and not a truncated one.
    pub fn save_to(&self, path: &Path) -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let text = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        let temp = path.with_extension("json.tmp");
        std::fs::write(&temp, text)?;
        std::fs::rename(&temp, path)
    }

    pub fn has(&self, folder: &Path) -> bool {
        self.builds.iter().any(|b| b.folder == folder)
    }

    /// Adds a build unless its folder is already there. Reports whether it was added.
    pub fn add(&mut self, build: Build) -> bool {
        if self.has(&build.folder) {
            return false;
        }
        self.builds.push(build);
        true
    }

    pub fn remove(&mut self, index: usize) -> Option<Build> {
        (index < self.builds.len()).then(|| self.builds.remove(index))
    }

    /// How many builds of a game.
    pub fn count(&self, game: Game) -> usize {
        self.builds.iter().filter(|b| b.game == game).count()
    }
}
