mod error;
mod map;
#[cfg(test)]
mod tests;

use crate::map::reader::MapReader;
use std::path::PathBuf;

pub struct GameReader {
    game_path: PathBuf,
    sources_path: PathBuf,
}

impl GameReader {
    pub fn connect(game_path: &str, sources_path: &str) -> Self {
        GameReader {
            game_path: PathBuf::from(game_path),
            sources_path: PathBuf::from(sources_path),
        }
    }

    pub fn game_path(&self) -> &PathBuf {
        &self.game_path
    }

    pub fn sources_path(&self) -> &PathBuf {
        &self.sources_path
    }

    pub fn maps(&self) -> MapReader {
        MapReader::from(self)
    }
}
