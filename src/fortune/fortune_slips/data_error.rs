use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Could not find key '{}'", key)]
    KeyNotFound { key: String },
    #[error("Out of index error; tried to retrieve")]
    OutOfBounds { idx: usize },
}
