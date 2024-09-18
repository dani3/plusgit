use std::error::Error;
use std::fmt::Debug;

#[derive(thiserror::Error)]
pub enum PlusGitError {
    #[error("Command run outside of a Plusgit repository")]
    NotInsideRepoError,

    #[error("Error writing or reading file")]
    IoError(#[from] std::io::Error),

    #[error("Object not found")]
    ObjectNotFoundError,
}

impl Debug for PlusGitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}
