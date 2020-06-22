use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("AWS API call error: {0:}")]
    ParseRegionError(#[from] rusoto_signature::region::ParseRegionError),

    #[error("TlsError: {0:?}")]
    TlsError(#[from] rusoto_core::request::TlsError),

    #[error("aws credentials error: {0:?}")]
    AwsCredentialsError(#[from] rusoto_credential::CredentialsError),

    #[error("Yaml format error: {0:?}")]
    CacheYamlFormatError(#[from] yaml_rust::emitter::EmitError),

    #[error("Unable to write file at {0:?}.")]
    UnableToWriteFileError(String),

    #[error("{0:}")]
    ArgumentError(String),

    #[error("rusoto error: {0:}")]
    RusotoError(String),

    #[error("xml error")]
    XmlError,

    #[error("setting error {0}")]
    SettingError(String),

    #[error("json error")]
    JsonError(#[from] serde_json::error::Error),

    #[error("duplicated xml tag {0}")]
    DuplicatedXmlTag(String),

    #[error("parameter error {0}")]
    ParameterError(String),

    #[error("std::io::error {0:?}")]
    TermError(#[from] std::io::Error),
}
