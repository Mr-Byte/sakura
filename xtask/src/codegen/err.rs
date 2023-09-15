use miette::{self, Diagnostic};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("codegen encountered one or more errors")]
pub(crate) struct CodegenError {
    #[related]
    pub(crate) others: Vec<miette::Error>,
}

impl FromIterator<miette::Result<()>> for CodegenError {
    fn from_iter<T: IntoIterator<Item = miette::Result<()>>>(iter: T) -> Self {
        let errs = iter.into_iter().filter_map(|result| result.err()).collect();

        Self { others: errs }
    }
}

impl Into<miette::Result<()>> for CodegenError {
    fn into(self) -> miette::Result<()> {
        if self.others.len() == 0 {
            return Ok(());
        }

        Err(self.into())
    }
}
