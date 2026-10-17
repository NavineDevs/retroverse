//! A kind of client. The base project defines the shape and nothing else.

use std::path::Path;

use crate::catalog::Game;
use crate::detect::Detected;

pub trait ClientFamily: Sync {
    fn game(&self) -> Game;
    fn name(&self) -> &'static str;
    fn label(&self, found: &Detected) -> String;
}

pub struct Fortnite;
pub struct Roblox;

impl ClientFamily for Fortnite {
    fn game(&self) -> Game {
        Game::Fortnite
    }
    fn name(&self) -> &'static str {
        "Fortnite"
    }
    fn label(&self, found: &Detected) -> String {
        found.version.clone()
    }
}

impl ClientFamily for Roblox {
    fn game(&self) -> Game {
        Game::Roblox
    }
    fn name(&self) -> &'static str {
        "Roblox"
    }
    fn label(&self, found: &Detected) -> String {
        found.version.clone()
    }
}

pub fn all() -> &'static [&'static dyn ClientFamily] {
    &[&Fortnite, &Roblox]
}

pub fn for_game(game: Game) -> &'static dyn ClientFamily {
    match game {
        Game::Fortnite => &Fortnite,
        Game::Roblox => &Roblox,
    }
}

/// Always `None`: nothing is recognised in the base project.
pub fn identify(_folder: &Path) -> Option<(&'static dyn ClientFamily, Detected)> {
    None
}
