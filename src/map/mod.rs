pub mod arena;
pub mod reader;

use crate::error::GameReadError;
use crate::map::arena::ArenaDefinition;
use crate::map::reader::XmlMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Map {
    sources_path: PathBuf,
    pub name: String,
    pub is_development: bool,
}

impl Map {
    pub fn new(xml_map: &XmlMap, sources_path: &PathBuf) -> Map {
        let is_development = if let Some(dev) = &xml_map.is_development {
            dev == "True"
        } else {
            false
        };
        Map {
            name: xml_map.name.clone(),
            is_development,
            sources_path: sources_path.clone(),
        }
    }

    pub fn arena(&self) -> Result<ArenaDefinition, GameReadError> {
        match self.is_development {
            true => Err(GameReadError::ArenaDefinitionNotFound(self.name.clone())),
            false => ArenaDefinition::parse(&self.sources_path, &self.name),
        }
    }
}
