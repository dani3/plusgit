/// # Repository
///
/// This module consists of functions that operate on the repository itself.
use std::path::PathBuf;

/// Return the 'objects' directory.
pub fn objects_dir() -> PathBuf {
    let mut object_path = PathBuf::new();
    object_path.push(crate::PLUSGIT_DIR);
    object_path.push(crate::OBJECTS_DIR);
    return object_path;
}
