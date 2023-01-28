use serde_variant::UnsupportedType;

#[derive(thiserror::Error, Debug)]
pub enum GameReadError {
    #[error("i/o error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Error during the xml parsing: {0}")]
    XmlParseError(#[from] serde_xml_rs::Error),

    #[error("The specified map {0} doesn't exists")]
    MapNotFound(String),

    #[error("The specified map {0} doesn't contains arena definition")]
    ArenaDefinitionNotFound(String),

    #[error("Localization catalog not found")]
    LocalizationCatalogNotFound(#[from] gettext::Error),

    #[error("Invalid enum variant")]
    InvalidVariant(#[from] UnsupportedType),

    #[error("Character {0} not found in string {1}")]
    CharacterNotFound(char, String),
}
