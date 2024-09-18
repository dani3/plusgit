/// # (Git) Init
///
/// This module implements all the logic for supporting the `init` Git command.
use std::{
    fs::File,
    io::{self},
    path::{Path, PathBuf},
};

use crate::error::PlusGitError;
use crate::error::PlusGitError::NotInsideRepoError;
use anyhow::Result;
use sha2::{Digest, Sha256};

/// Helper function that checks if we are running inside a plusgit repository.
fn is_inside_repo() -> bool {
    let current_dir = std::env::current_dir().unwrap();
    current_dir
        .read_dir()
        .unwrap()
        .any(|f| f.unwrap().file_name() == crate::PLUSGIT_DIR)
}

/// Implementation of the `init` plusgit command that creates a new empty repository.
/// It receives as a parameter the path where the repository should be created.
pub fn init(path: &String) -> Result<(), io::Error> {
    let plusgit_dir_str: String = format!("{}/{}", path, crate::PLUSGIT_DIR);
    let plusgit_dir: &Path = Path::new(&plusgit_dir_str);
    let objects_dir: PathBuf = Path::new(&plusgit_dir_str).join(crate::OBJECTS_DIR);

    // create the .plusgit directory
    std::fs::create_dir_all(plusgit_dir)?;
    // create the .plusgit/objects directory
    std::fs::create_dir(objects_dir)?;

    println!(
        "{}",
        format!(
            "Initialized empty plusgit repository in {}",
            plusgit_dir.canonicalize().unwrap().display()
        )
    );
    Ok(())
}

/// Compute object ID and create an object from a file.
/// See https://git-scm.com/docs/git-hash-object
pub fn hash_object(filepath: &String) -> Result<(), PlusGitError> {
    if !is_inside_repo() {
        return Err(NotInsideRepoError);
    }

    // Hash the file using SHA256.
    let bytes: Vec<u8> = std::fs::read(filepath)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hash = format!("{:X}", hasher.finalize());

    // Store the file using the hash as the filename.
    let mut object_path = PathBuf::new();
    object_path.push(crate::PLUSGIT_DIR);
    object_path.push(crate::OBJECTS_DIR);
    object_path.push(&hash);
    File::create(object_path)?;
    println!("Hash of '{}' is '{}'", filepath, hash);

    Ok(())
}
