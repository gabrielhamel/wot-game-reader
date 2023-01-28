use crate::errors::GameReadError;
use crate::GameReader;
use gettext::Catalog;
use serde::Serialize;
use serde_variant::to_variant_name;
use std::fs::File;
use std::path::PathBuf;

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
    pub fn get(&self, game_reader: &GameReader) -> Result<Catalog, GameReadError> {
        let path = match self {
            LocalizationCatalog::Arenas => ["res", "text", "lc_messages", "arenas.mo"]
                .iter()
                .collect::<PathBuf>(),
            LocalizationCatalog::Nations => ["res", "text", "lc_messages", "nations.mo"]
                .iter()
                .collect::<PathBuf>(),
            LocalizationCatalog::Tanks(nation) => [
                "res",
                "text",
                "lc_messages",
                &format!("{}_vehicles.mo", to_variant_name(nation)?),
            ]
            .iter()
            .collect::<PathBuf>(),
        };
        let filepath = game_reader.game_path.join(path);
        let final_path = File::open(filepath)?;
        Ok(Catalog::parse(final_path)?)
    }
}

impl LocalizationReader {
    pub fn translate(
        &self,
        catalog: LocalizationCatalog,
        key: &str,
    ) -> Result<String, GameReadError> {
        let translator = catalog.get(&self.game_reader)?;
        let translation = translator.gettext(key).to_string();
        if translation == key {
            // Use the short name. Some tanks doesn't need their prefix (STB-1)
            let short_name = &key[key
                .find('_')
                .ok_or(GameReadError::CharacterNotFound('_', key.to_string()))?
                + 1..];
            Ok(translator.gettext(short_name).to_string())
        } else {
            Ok(translation)
        }
    }
}
