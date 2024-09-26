use crate::object::ObjectKind;
/// # Plusgit
///
/// This module implements all the commands of Plusgit.
use crate::{error::PlusGitError, repo};
use crate::{error::PlusGitError::NotInsideRepoError, object};
use anyhow::Result;

/// Implementation of the `init` plusgit command that creates a new empty repository.
/// It receives as a parameter the path where the repository should be created.
pub fn init(path: &String) -> Result<(), PlusGitError> {
    let path = repo::init(path)?;

    println!(
        "{}",
        format!("Initialized empty plusgit repository in {}", path)
    );
    Ok(())
}

/// Compute object ID and create an object from a file.
/// See https://git-scm.com/docs/git-hash-object
pub fn hash_object(filepath: &String, kind: ObjectKind) -> Result<(), PlusGitError> {
    if !repo::is_inside_repo() {
        return Err(NotInsideRepoError);
    }

    let hash = object::hash(filepath, kind)?;
    println!("Hash of '{}' is '{}'", filepath, hash);

    Ok(())
}

/// Provide contents or details of repository objects
/// See https://git-scm.com/docs/git-cat-file
pub fn cat_file(oid: &String, kind: ObjectKind) -> Result<(), PlusGitError> {
    let object = object::from_hash(oid, kind)?;
    println!("{}", object.as_str());
    Ok(())
}
