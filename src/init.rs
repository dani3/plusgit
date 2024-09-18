use std::path::Path;

/// # (Git) Init
///
/// This module implements all the logic for supporting the `init` Git command.

pub fn run(path: &String) {
    let full_path: String = format!("{}/{}", path, crate::PLUSGIT_DIR);
    let dir: &Path = Path::new(&full_path);

    match std::fs::create_dir_all(dir) {
        Ok(()) => {
            println!(
                "{}",
                format!(
                    "Initialized empty plusgit repository in {}",
                    dir.canonicalize().unwrap().display()
                )
            );
        }
        Err(e) => {
            println!("An error occurred when initializing the repository");
            println!("{e}");
        }
    }
}
