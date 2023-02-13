use crate::errors::GameReadError;
use crate::game_reader::GameReader;
use serde::Serialize;
use serde_variant::to_variant_name;
use std::fs::File;
use std::path::PathBuf;
use poreader::PoParser;

pub struct LocalizationReader {
    game_reader: GameReader,
}

impl From<&GameReader> for LocalizationReader {
    fn from(game_reader: &GameReader) -> Self {
        LocalizationReader {
            game_reader: game_reader.clone(),
        }
    }
}

#[derive(Serialize)]
pub enum Nation {
    Multinational,
    Gb,
    Germany,
    Poland,
    Italy,
    Sweden,
    Ussr,
    Japan,
    Igr,
    China,
    Czech,
    France,
    Usa,
}

pub enum LocalizationCatalog {
    Arenas,
    Nations,
    Tanks(Nation),
}

impl LocalizationCatalog {
    pub fn get(&self, game_reader: &GameReader) -> Result<File, GameReadError> {
        let path = match self {
            LocalizationCatalog::Arenas => ["res", "text", "lc_messages", "arenas.po"]
                .iter()
                .collect::<PathBuf>(),
            LocalizationCatalog::Nations => ["res", "text", "lc_messages", "nations.po"]
                .iter()
                .collect::<PathBuf>(),
            LocalizationCatalog::Tanks(nation) => [
                "res",
                "text",
                "lc_messages",
                &format!("{}_vehicles.po", to_variant_name(nation)?),
            ]
            .iter()
            .collect::<PathBuf>(),
        };
        let filepath = game_reader.sources_path().join(path);
        Ok(File::open(filepath)?)
    }
}

impl LocalizationReader {
    pub fn fetch(&self, catalog: LocalizationCatalog, key: &str) -> Result<String, GameReadError> {
        let file = catalog.get(&self.game_reader)?;
        let parser = PoParser::new();
        let mut reader = parser.parse(file)?;
        let found = reader.find_map(|e| {
            match e {
                Ok(unit) => {
                    if unit.message().get_id() == key {
                        Some(unit.message().get_text().to_string())
                    } else {
                        None
                    }
                },
                _ => None,
            }
        });
        found.ok_or(GameReadError::LocalizationKeyNotFound)
    }
}
