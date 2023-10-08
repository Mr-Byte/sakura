use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum CodegenError {
    #[error("{} has been updated, please re-run `cargo tests`", .0)]
    CodeUpdated(String),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
