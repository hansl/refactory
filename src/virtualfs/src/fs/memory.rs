use crate::error::{Error, IoError, Result};
use crate::fs::FileSystem;
use crate::path::{Component, OwnedPath, Path};
use std::collections::BTreeMap;
use thiserror::Error as DeriveError;

#[derive(Debug, DeriveError)]
pub enum MemFsError {
    #[error("Path {0} is not a directory (but was expected to be).")]
    IsNotADirectory(OwnedPath),

    #[error("Path {0} is not a file (but was expected to be).")]
    IsNotAFile(OwnedPath),

    #[error("Path {0} points to a directory (was expecting a file).")]
    PathIsNotAFile(OwnedPath),
}

pub enum MemFsEntryKind {
    Directory {
        entries: BTreeMap<String, MemFsEntry>,
    },
    File {
        content: Vec<u8>,
    },
}

pub struct MemFsEntry {
    path: OwnedPath,
    kind: MemFsEntryKind,
}

impl MemFsEntry {
    pub fn dir(path: OwnedPath) -> Self {
        Self {
            path,
            kind: MemFsEntryKind::Directory {
                entries: Default::default(),
            },
        }
    }

    pub fn file(path: OwnedPath) -> Self {
        Self {
            path,
            kind: MemFsEntryKind::File {
                content: Default::default(),
            },
        }
    }

    pub fn delete<N: AsRef<str>>(&mut self, name: N) -> Result<()> {
        match &mut self.kind {
            MemFsEntryKind::Directory { entries } => entries
                .remove(name.as_ref())
                .map(|_| ())
                .ok_or_else(|| Error::Io(IoError::NotFound)),
            _ => Err(Error::custom(MemFsError::IsNotADirectory(
                self.path.clone(),
            ))),
        }
    }

    pub fn as_dir(&self) -> Result<&Self> {
        match &self.kind {
            MemFsEntryKind::Directory { .. } => Ok(self),
            _ => Err(Error::custom(MemFsError::IsNotADirectory(
                self.path.clone(),
            ))),
        }
    }

    pub fn as_file(&self) -> Result<&Self> {
        match &self.kind {
            MemFsEntryKind::File { .. } => Ok(self),
            _ => Err(Error::custom(MemFsError::IsNotAFile(self.path.clone()))),
        }
    }

    pub fn as_dir_mut(&mut self) -> Result<&mut Self> {
        match &self.kind {
            MemFsEntryKind::Directory { .. } => Ok(self),
            _ => Err(Error::custom(MemFsError::IsNotADirectory(
                self.path.clone(),
            ))),
        }
    }

    pub fn as_file_mut(&mut self) -> Result<&mut Self> {
        match &self.kind {
            MemFsEntryKind::File { .. } => Ok(self),
            _ => Err(Error::custom(MemFsError::IsNotAFile(self.path.clone()))),
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self.kind, MemFsEntryKind::Directory { .. })
    }

    pub fn is_file(&self) -> bool {
        matches!(self.kind, MemFsEntryKind::File { .. })
    }

    pub fn read(&self) -> Result<&[u8]> {
        match &self.kind {
            MemFsEntryKind::File { content } => Ok(content),
            _ => Err(Error::custom(MemFsError::PathIsNotAFile(self.path.clone()))),
        }
    }

    pub fn write(&mut self, new_content: &[u8]) -> Result<()> {
        match &mut self.kind {
            MemFsEntryKind::File { content } => {
                *content = new_content.to_vec();
                Ok(())
            }
            _ => Err(Error::custom(MemFsError::IsNotAFile(self.path.clone()))),
        }
    }

    pub fn get_entry(&self, name: &Component) -> Result<&MemFsEntry> {
        match &self.kind {
            MemFsEntryKind::Directory { entries } => entries
                .get(&name.to_string())
                .ok_or_else(|| Error::Io(IoError::NotFound)),
            _ => Err(Error::custom(MemFsError::IsNotADirectory(
                self.path.clone(),
            ))),
        }
    }

    pub fn get_entry_mut(&mut self, name: &Component) -> Result<&mut MemFsEntry> {
        match &mut self.kind {
            MemFsEntryKind::Directory { entries } => entries
                .get_mut(&name.to_string())
                .ok_or_else(|| Error::Io(IoError::NotFound)),
            _ => Err(Error::custom(MemFsError::IsNotADirectory(
                self.path.clone(),
            ))),
        }
    }

    pub fn get_or_create_dir(&mut self, name: &Component) -> Result<&mut MemFsEntry> {
        let full_path = self.path.join(name);

        match &mut self.kind {
            MemFsEntryKind::Directory { entries } => Ok(entries
                .entry(name.to_string())
                .or_insert_with(|| MemFsEntry::dir(full_path.clone())))
            .and_then(|entry| {
                if !entry.is_dir() {
                    Err(Error::custom(MemFsError::IsNotADirectory(full_path)))
                } else {
                    Ok(entry)
                }
            }),

            _ => Err(Error::custom(MemFsError::IsNotADirectory(full_path))),
        }
    }

    pub fn get_or_create_file(&mut self, name: &Component) -> Result<&mut MemFsEntry> {
        let full_path = self.path.join(name);

        match &mut self.kind {
            MemFsEntryKind::Directory { entries } => Ok(entries
                .entry(name.to_string())
                .or_insert_with(|| MemFsEntry::file(full_path.clone())))
            .and_then(|entry| {
                if !entry.is_file() {
                    Err(Error::custom(MemFsError::IsNotADirectory(full_path)))
                } else {
                    Ok(entry)
                }
            }),
            _ => Err(Error::custom(MemFsError::IsNotADirectory(full_path))),
        }
    }
}

pub struct MemoryFileSystem {
    root: MemFsEntry,
}

impl Default for MemoryFileSystem {
    fn default() -> Self {
        Self {
            root: MemFsEntry {
                path: OwnedPath::root(),
                kind: MemFsEntryKind::Directory {
                    entries: Default::default(),
                },
            },
        }
    }
}

impl MemoryFileSystem {
    pub fn empty() -> Self {
        Default::default()
    }

    pub fn get_dir_or_create(&mut self, path: &Path) -> Result<&mut MemFsEntry> {
        path.iter().fold(Ok(&mut self.root), |acc, entry_name| {
            acc?.get_or_create_dir(&entry_name)
        })
    }

    pub fn get_file_or_create(&mut self, path: &Path) -> Result<&mut MemFsEntry> {
        let dir = self.get_dir_or_create(&path.parent().unwrap())?;
        dir.get_or_create_file(&path.basename().unwrap())
    }

    pub fn get_entry(&self, path: &Path) -> Result<&MemFsEntry> {
        path.iter().fold(Ok(&self.root), |acc, entry_name| {
            acc?.get_entry(&entry_name)
        })
    }

    pub fn get_dir(&self, path: &Path) -> Result<&MemFsEntry> {
        self.get_entry(path)?.as_dir()
    }

    pub fn get_file(&self, path: &Path) -> Result<&MemFsEntry> {
        self.get_entry(path)?.as_file()
    }

    pub fn get_entry_mut(&mut self, path: &Path) -> Result<&mut MemFsEntry> {
        path.iter().fold(Ok(&mut self.root), |acc, entry_name| {
            acc?.get_entry_mut(&entry_name)
        })
    }

    pub fn get_dir_mut(&mut self, path: &Path) -> Result<&mut MemFsEntry> {
        self.get_entry_mut(path)?.as_dir_mut()
    }

    pub fn get_file_mut(&mut self, path: &Path) -> Result<&mut MemFsEntry> {
        self.get_entry_mut(path)?.as_file_mut()
    }
}

impl FileSystem for MemoryFileSystem {
    fn read<P: AsRef<Path>>(&self, path: P) -> Result<&[u8]> {
        self.get_file(path.as_ref())?.read()
    }

    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        self.get_entry(path.as_ref()).is_ok()
    }

    fn delete<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.get_entry_mut(
            path.as_ref()
                .parent()
                .ok_or_else(|| Error::Io(IoError::NotFound))?,
        )?
        .delete(
            path.as_ref()
                .basename()
                .ok_or_else(|| Error::Io(IoError::NotFound))?,
        )
    }

    fn rename<From: AsRef<Path>, To: AsRef<Path>>(&self, from: From, to: To) -> Result<()> {
        unimplemented!()
    }

    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&mut self, path: P, content: C) -> Result<()> {
        self.get_file_or_create(path.as_ref())?
            .write(content.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn base() -> Result<()> {
        let mut fs = MemoryFileSystem::default();
        fs.write("/hello.txt", "blue")?;
        assert_eq!(fs.read("/hello.txt")?, b"blue");
        assert!(fs.exists("/hello.txt"));
        assert!(!fs.exists("/hello_UNKNOWN.txt"));

        fs.delete("/hello.txt")?;
        assert!(!fs.exists("/hello.txt"));
        Ok(())
    }

    #[test]
    fn base_err() -> Result<()> {
        let fs = MemoryFileSystem::default();
        assert!(fs.read("/does/not/exist").is_err());
        Ok(())
    }
}
