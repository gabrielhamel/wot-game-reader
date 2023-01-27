use crate::error::GameReadError;
use crate::map::Map;
use crate::GameReader;
use serde::{Deserialize, Serialize};
use serde_xml_rs as xml;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XmlMap {
    id: i32,
    pub name: String,
    pub is_development: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Root {
    pub map: Vec<XmlMap>,
}

pub struct MapReader {
    sources_path: PathBuf,
}

impl From<&GameReader> for MapReader {
    fn from(game_reader: &GameReader) -> Self {
        MapReader {
            sources_path: game_reader.sources_path().clone(),
        }
    }
}

impl MapReader {
    pub fn list(&self) -> Result<Vec<Map>, GameReadError> {
        let path = self
            .sources_path
            .join("res")
            .join("scripts")
            .join("arena_defs")
            .join("_list_.xml");
        let file = File::open(path)?;
        let parsed: Root = xml::from_reader(file)?;
        Ok(parsed
            .map
            .into_iter()
            .map(|xml| Map::new(&xml, &self.sources_path))
            .collect())
    }

    pub fn get_by_name(&self, map_name: &str) -> Result<Map, GameReadError> {
        self.list()?
            .into_iter()
            .find(|m| m.name == map_name)
            .ok_or(GameReadError::MapNotFound(String::from(map_name)))
    }
}
