use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum Base62Error {
    #[error("unexpected input length")]
    InvalidInput,

    #[error("error encoding value")]
    EncodeError,

    #[error("error decoding value")]
    DecodeError,
}
