#[derive(thiserror::Error, Debug)]
pub enum GameReadError {
    #[error("Unable to perform io operation on the requested file")]
    IoError(#[from] std::io::Error),

    #[error("Error during the xml parsing")]
    XmlParseError(#[from] serde_xml_rs::Error),

    #[error("The specified map {0} doesn't exists")]
    MapNotFound(String),

    #[error("The specified map {0} doesn't contains arena definition")]
    ArenaDefinitionNotFound(String),
}
