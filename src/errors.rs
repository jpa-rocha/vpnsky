use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrors {
    #[error(transparent)]
    FolderCreation(#[from] FolderCreationError),

    #[error(transparent)]
    OpenFolder(#[from] OpenFolderError),

    #[error(transparent)]
    Logger(#[from] LoggerError),

    #[error(transparent)]
    Config(#[from] ConfigurationError),

    #[error(transparent)]
    Input(#[from] GetInputError),
}

#[derive(Error, Debug)]
pub enum FolderCreationError {
    #[error("Folder already exists")]
    FolderAlreadyExists,

    #[error("Could not create folder")]
    CouldNotCreateFolder,

    #[error("Could not read folder")]
    CouldNotRead,
}

#[derive(Error, Debug)]
pub enum OpenFolderError {
    #[error("Could not open folder")]
    CouldNotOpenFolder,
}

#[derive(Error, Debug)]
pub enum LoggerError {
    #[error("Could not initialize logger: {0}")]
    CouldNotInitializeLogger(#[from] spdlog::Error),

    #[error("Logger lever provided not supported")]
    UnsupportedLoggerLevel,
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Could not initialize config: {0}")]
    CouldNotLoadConfig(#[from] config::ConfigError),

    #[error("Could not deserialize config")]
    CouldNotDeserializeConfig,
}

#[derive(Error, Debug)]
pub enum GetInputError {
    #[error("Could not make request: {0}")]
    MakingRequest(#[from] reqwest::Error),

    #[error("Unable to read response body: {0}")]
    ReadingResponseBody(#[from] std::io::Error),

    #[error("Could not find input")]
    InputNotFound,

    #[error("Could not return input")]
    CouldNotReturnInput,

    #[error("Could not create input file")]
    CouldNotCreateInputFile,
}
