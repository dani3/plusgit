use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::str;
use std::str::FromStr;

use crate::error::PlusGitError;
use crate::repo;

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum ObjectKind {
    Blob = 1,
    Tree,
    Tag,
    Commit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    pub kind: ObjectKind,
    pub bytes: Vec<u8>,
}

impl Object {
    pub fn new(kind: ObjectKind, bytes: Vec<u8>) -> Self {
        Object { kind, bytes }
    }

    /// Return the `bytes` field as a UTF-8 str.
    pub fn as_str(self: &Object) -> &str {
        str::from_utf8(&self.bytes).unwrap()
    }
}

impl FromStr for ObjectKind {
    type Err = ();

    fn from_str(input: &str) -> Result<ObjectKind, Self::Err> {
        match input {
            "blob" => Ok(ObjectKind::Blob),
            "tag" => Ok(ObjectKind::Tag),
            "tree" => Ok(ObjectKind::Tree),
            "commit" => Ok(ObjectKind::Commit),
            _ => Err(()),
        }
    }
}

pub fn hash(file: &String, kind: ObjectKind) -> Result<String, PlusGitError> {
    let bytes: Vec<u8> = std::fs::read(file)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash = format!("{:X}", hasher.finalize());

    let object: Object = Object::new(kind, bytes);

    let mut object_path = repo::objects_dir();
    object_path.push(&hash);
    let mut file = File::create(object_path)?;
    file.write(serde_json::to_string(&object).unwrap().as_bytes())?;

    Ok(hash)
}

pub fn from_hash(hash: &String) -> Result<Object, PlusGitError> {
    if let Some(f) = repo::objects_dir()
        .read_dir()
        .unwrap()
        .find(|f| f.as_ref().unwrap().file_name().to_str().unwrap() == hash)
    {
        let raw: String = std::fs::read_to_string(f?.path())?;
        Ok(serde_json::from_str(&raw).unwrap())
    } else {
        Err(PlusGitError::ObjectNotFoundError)
    }
}
