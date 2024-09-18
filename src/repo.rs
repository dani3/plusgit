/// # Repository
///
/// This module consists of functions that operate on the repository itself.
use crate::error::PlusGitError;
use std::path::{Path, PathBuf};

pub const PLUSGIT_DIR: &'static str = ".plusgit";
pub const OBJECTS_DIR: &'static str = "objects";

pub fn init(path: &String) -> Result<String, PlusGitError> {
    let plusgit_dir_str: String = format!("{}/{}", path, PLUSGIT_DIR);
    let plusgit_dir: &Path = Path::new(&plusgit_dir_str);
    let objects_dir: PathBuf = Path::new(&plusgit_dir_str).join(OBJECTS_DIR);

    // create the .plusgit directory
    std::fs::create_dir_all(plusgit_dir)?;
    // create the .plusgit/objects directory
    std::fs::create_dir(objects_dir)?;
    Ok(format!("{}", plusgit_dir.canonicalize().unwrap().display()))
}

/// Return the 'objects' directory.
pub fn objects_dir() -> PathBuf {
    let mut object_path = PathBuf::new();
    object_path.push(PLUSGIT_DIR);
    object_path.push(OBJECTS_DIR);
    return object_path;
}

/// Helper function that checks if we are running inside a plusgit repository.
pub fn is_inside_repo() -> bool {
    let current_dir = std::env::current_dir().unwrap();
    current_dir
        .read_dir()
        .unwrap()
        .any(|f| f.unwrap().file_name() == PLUSGIT_DIR)
}
