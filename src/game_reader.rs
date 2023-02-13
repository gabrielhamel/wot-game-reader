use crate::localization::LocalizationReader;
use crate::map::reader::MapReader;
use std::path::PathBuf;

#[derive(Clone)]
pub struct GameReader {
    sources_path: PathBuf,
}

impl GameReader {
    pub fn connect(sources_path: &str) -> Self {
        GameReader {
            sources_path: PathBuf::from(sources_path),
        }
    }

    pub fn sources_path(&self) -> &PathBuf {
        &self.sources_path
    }

    pub fn maps(&self) -> MapReader {
        MapReader::from(self)
    }

    pub fn localization(&self) -> LocalizationReader {
        LocalizationReader::from(self)
    }
}
