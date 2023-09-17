use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum CodegenError {
    #[error("{} has been updated, please re-run `cargo test`", .0)]
    CodeUpdated(String),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
