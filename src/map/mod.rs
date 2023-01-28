pub mod arena;
pub mod reader;

use crate::errors::GameReadError;
use crate::map::arena::ArenaDefinition;
use crate::map::reader::XmlMap;
use crate::GameReader;
use crate::localization::LocalizationCatalog::Arenas;

pub struct Map {
    game_reader: GameReader,
    pub name: String,
    pub is_development: bool,
}

impl Map {
    pub fn new(xml_map: &XmlMap, game_reader: &GameReader) -> Map {
        let is_development = if let Some(dev) = &xml_map.is_development {
            dev == "True"
        } else {
            false
        };
        Map {
            name: xml_map.name.clone(),
            is_development,
            game_reader: game_reader.clone(),
        }
    }

    pub fn arena(&self) -> Result<ArenaDefinition, GameReadError> {
        match self.is_development {
            true => Err(GameReadError::ArenaDefinitionNotFound(self.name.clone())),
            false => ArenaDefinition::parse(&self.game_reader, &self.name),
        }
    }

    pub fn display_name(&self) -> Result<String, GameReadError> {
        self.game_reader.localization().translate(Arenas, &format!("{}/name", &self.name))
    }
}
