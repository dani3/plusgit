use std::{
    io,
    path::{Path, PathBuf},
};

use anyhow::Result;

/// # (Git) Init
///
/// This module implements all the logic for supporting the `init` Git command.

pub fn init(path: &String) -> Result<(), io::Error> {
    let plusgit_dir_str: String = format!("{}/{}", path, crate::PLUSGIT_DIR);
    let plusgit_dir: &Path = Path::new(&plusgit_dir_str);
    let objects_dir: PathBuf = Path::new(&plusgit_dir_str).join(crate::OBJECTS_DIR);

    std::fs::create_dir_all(plusgit_dir)?;
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
