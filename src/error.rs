use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The prerequisite checks failed.")]
    ChecksFailed,
    #[error("Failed to load the fortune settings.")]
    FortuneSettingsLoadFailure(String),
    #[error("Failed to generate the fortune slips.")]
    PdfGenerateFailure(String),
}
