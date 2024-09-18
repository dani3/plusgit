use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;

use crate::error::PlusGitError;
use crate::repo;

pub enum ObjectKind {
    Blob,
}

pub fn hash(file: &String) -> Result<String, PlusGitError> {
    let bytes: Vec<u8> = std::fs::read(file)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash = format!("{:X}", hasher.finalize());

    let mut object_path = repo::objects_dir();
    object_path.push(&hash);
    let mut file = File::create(object_path)?;
    file.write(&bytes)?;

    Ok(hash)
}

pub fn from_hash(hash: &String) -> Result<String, PlusGitError> {
    if let Some(f) = repo::objects_dir()
        .read_dir()
        .unwrap()
        .find(|f| f.as_ref().unwrap().file_name().to_str().unwrap() == hash)
    {
        Ok(std::fs::read_to_string(f?.path())?)
    } else {
        Err(PlusGitError::ObjectNotFoundError)
    }
}
