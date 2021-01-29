use crate::error::{Error, IoError, Result};
use crate::fs::FileSystem;
use crate::path::{Component, OwnedPath, Path};
use std::collections::BTreeMap;
use std::fmt::Formatter;
use thiserror::Error as DeriveError;

type MemFsResult<X> = std::result::Result<X, MemFsError>;

pub struct InternalError(Box<dyn FnOnce(&Path) -> MemFsError + Send + Sync>);

impl std::fmt::Debug for InternalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Internal Error.")
    }
}

#[derive(Debug, DeriveError)]
pub enum MemFsError {
    #[error("Path {0} could not be found.")]
    PathNotFound(OwnedPath),

    #[error("Path {0} is not a directory (but was expected to be).")]
    PathIsNotADirectory(OwnedPath),

    #[error("Path {0} points to a directory (was expecting a file).")]
    PathIsNotAFile(OwnedPath),

    #[error("Internal Error (unexpected)")]
    _Internal(InternalError),
}

#[derive(Clone, Debug)]
enum MemFsEntryKind {
    Directory {
        entries: BTreeMap<String, MemFsEntry>,
    },
    File {
        content: Vec<u8>,
    },
}

fn not_found() -> MemFsError {
    MemFsError::_Internal(InternalError(Box::new(|path| {
        MemFsError::PathNotFound(path.to_owned())
    })))
}

fn expected_a_file() -> MemFsError {
    MemFsError::_Internal(InternalError(Box::new(|path| {
        MemFsError::PathIsNotAFile(path.to_owned())
    })))
}

fn expected_a_dir() -> MemFsError {
    MemFsError::_Internal(InternalError(Box::new(|path| {
        MemFsError::PathIsNotADirectory(path.to_owned())
    })))
}

fn map_memfs_error(path: &Path, err: MemFsError) -> Error {
    match err {
        MemFsError::_Internal(InternalError(f)) => Error::custom(f(path)),
        x => Error::custom(x),
    }
}

#[derive(Clone, Debug)]
struct MemFsEntry {
    pub kind: MemFsEntryKind,
}

impl MemFsEntry {
    pub fn dir() -> Self {
        Self {
            kind: MemFsEntryKind::Directory {
                entries: Default::default(),
            },
        }
    }

    pub fn file(content: Vec<u8>) -> Self {
        Self {
            kind: MemFsEntryKind::File { content },
        }
    }

    pub fn delete<N: AsRef<str>>(&mut self, name: N) -> MemFsResult<MemFsEntry> {
        match &mut self.kind {
            MemFsEntryKind::Directory { entries } => {
                entries.remove(name.as_ref()).ok_or_else(not_found)
            }
            _ => Err(not_found()),
        }
    }

    pub fn set_or_replace_entry(&mut self, name: String, entry: MemFsEntry) -> MemFsResult<()> {
        match &mut self.kind {
            MemFsEntryKind::Directory { entries } => {
                let kind = entry.kind;

                entries
                    .entry(name)
                    .and_modify(|e| e.kind = kind.clone())
                    .or_insert_with(|| MemFsEntry { kind });

                Ok(())
            }
            _ => Err(expected_a_dir()),
        }
    }

    pub fn as_dir(&self) -> MemFsResult<&Self> {
        match &self.kind {
            MemFsEntryKind::Directory { .. } => Ok(self),
            _ => Err(expected_a_dir()),
        }
    }

    pub fn as_file(&self) -> MemFsResult<&Self> {
        match &self.kind {
            MemFsEntryKind::File { .. } => Ok(self),
            _ => Err(expected_a_file()),
        }
    }

    pub fn as_dir_mut(&mut self) -> MemFsResult<&mut Self> {
        match &self.kind {
            MemFsEntryKind::Directory { .. } => Ok(self),
            _ => Err(expected_a_dir()),
        }
    }

    pub fn as_file_mut(&mut self) -> MemFsResult<&mut Self> {
        match &self.kind {
            MemFsEntryKind::File { .. } => Ok(self),
            _ => Err(expected_a_file()),
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self.kind, MemFsEntryKind::Directory { .. })
    }

    pub fn is_file(&self) -> bool {
        matches!(self.kind, MemFsEntryKind::File { .. })
    }

    pub fn read(&self) -> MemFsResult<&[u8]> {
        match &self.kind {
            MemFsEntryKind::File { content } => Ok(content),
            _ => Err(expected_a_file()),
        }
    }

    pub fn write(&mut self, new_content: &[u8]) -> MemFsResult<()> {
        match &mut self.kind {
            MemFsEntryKind::File { content } => {
                *content = new_content.to_vec();
                Ok(())
            }
            _ => Err(expected_a_file()),
        }
    }

    pub fn get_entry(&self, name: &Component) -> MemFsResult<&MemFsEntry> {
        match &self.kind {
            MemFsEntryKind::Directory { entries } => {
                entries.get(&name.to_string()).ok_or_else(not_found)
            }
            _ => Err(expected_a_dir()),
        }
    }

    pub fn get_entry_mut(&mut self, name: &Component) -> MemFsResult<&mut MemFsEntry> {
        match &mut self.kind {
            MemFsEntryKind::Directory { entries } => {
                entries.get_mut(&name.to_string()).ok_or_else(not_found)
            }
            _ => Err(expected_a_dir()),
        }
    }

    pub fn get_or_create_dir(&mut self, name: &Component) -> MemFsResult<&mut MemFsEntry> {
        match &mut self.kind {
            MemFsEntryKind::Directory { entries } => Ok(entries
                .entry(name.to_string())
                .or_insert_with(|| MemFsEntry::dir()))
            .and_then(|entry| {
                if !entry.is_dir() {
                    Err(expected_a_dir())
                } else {
                    Ok(entry)
                }
            }),

            _ => Err(expected_a_dir()),
        }
    }

    pub fn get_or_create_file(&mut self, name: &Component) -> MemFsResult<&mut MemFsEntry> {
        match &mut self.kind {
            MemFsEntryKind::Directory { entries } => Ok(entries
                .entry(name.to_string())
                .or_insert_with(|| MemFsEntry::file(Default::default())))
            .and_then(|entry| {
                if !entry.is_file() {
                    Err(expected_a_dir())
                } else {
                    Ok(entry)
                }
            }),
            _ => Err(expected_a_dir()),
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

    fn get_dir_or_create(&mut self, path: &Path) -> Result<&mut MemFsEntry> {
        path.iter()
            .fold(Ok(&mut self.root), |acc, entry_name| {
                acc?.get_or_create_dir(&entry_name)
            })
            .map_err(|err| map_memfs_error(path, err))
    }

    fn get_file_or_create(&mut self, path: &Path) -> Result<&mut MemFsEntry> {
        let dir = self.get_dir_or_create(&path.parent().unwrap())?;
        dir.get_or_create_file(&path.basename().unwrap())
            .map_err(|err| map_memfs_error(path, err))
    }

    fn get_entry(&self, path: &Path) -> Result<&MemFsEntry> {
        path.iter().fold(Ok(&self.root), |acc, entry_name| {
            acc?.get_entry(&entry_name)
                .map_err(|err| map_memfs_error(path, err))
        })
    }

    fn get_dir(&self, path: &Path) -> Result<&MemFsEntry> {
        self.get_entry(path)?
            .as_dir()
            .map_err(|err| map_memfs_error(path, err))
    }

    fn get_file(&self, path: &Path) -> Result<&MemFsEntry> {
        self.get_entry(path)?
            .as_file()
            .map_err(|err| map_memfs_error(path, err))
    }

    fn get_entry_mut(&mut self, path: &Path) -> Result<&mut MemFsEntry> {
        path.iter().fold(Ok(&mut self.root), |acc, entry_name| {
            acc?.get_entry_mut(&entry_name)
                .map_err(|err| map_memfs_error(path, err))
        })
    }

    fn get_dir_mut(&mut self, path: &Path) -> Result<&mut MemFsEntry> {
        self.get_entry_mut(path)?
            .as_dir_mut()
            .map_err(|err| map_memfs_error(path, err))
    }

    fn get_file_mut(&mut self, path: &Path) -> Result<&mut MemFsEntry> {
        self.get_entry_mut(path)?
            .as_file_mut()
            .map_err(|err| map_memfs_error(path, err))
    }
}

impl FileSystem for MemoryFileSystem {
    fn read<P: AsRef<Path>>(&self, path: P) -> Result<&[u8]> {
        self.get_file(path.as_ref())?
            .read()
            .map_err(|err| map_memfs_error(path.as_ref(), err))
    }

    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        self.get_entry(path.as_ref()).is_ok()
    }

    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&mut self, path: P, content: C) -> Result<()> {
        self.get_file_or_create(path.as_ref())?
            .write(content.as_ref())
            .map_err(|err| map_memfs_error(path.as_ref(), err))
    }

    fn delete<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.get_entry_mut(
            path.as_ref()
                .parent()
                .ok_or_else(|| Error::Io(IoError::NotFound))?,
        )?
        .delete(
            &path
                .as_ref()
                .basename()
                .ok_or_else(|| Error::Io(IoError::NotFound))?,
        )
        .map(|_| ())
        .map_err(|err| map_memfs_error(path.as_ref(), err))
    }

    fn rename<From: AsRef<Path>, To: AsRef<Path>>(&mut self, from: From, to: To) -> Result<()> {
        let from_basename = from
            .as_ref()
            .basename()
            .ok_or_else(|| Error::Io(IoError::NotFound))?;
        let from_parent = from
            .as_ref()
            .parent()
            .ok_or_else(|| Error::Io(IoError::NotFound))?;
        let to_basename = to
            .as_ref()
            .basename()
            .ok_or_else(|| Error::Io(IoError::NotFound))?;
        let to_parent = to
            .as_ref()
            .parent()
            .ok_or_else(|| Error::Io(IoError::NotFound))?;
        let from_dir = self.get_entry_mut(from_parent)?;

        let content = from_dir
            .delete(from_basename)
            .map_err(|err| map_memfs_error(from.as_ref(), err))?;

        let to_dir = self.get_dir_or_create(to_parent)?;
        to_dir
            .set_or_replace_entry(to_basename.to_string(), content)
            .map_err(|err| map_memfs_error(to.as_ref(), err))?;

        Ok(())
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
    fn rename() -> Result<()> {
        let mut fs = MemoryFileSystem::default();
        fs.write("/a/b/hello.txt", "blue")?;
        assert!(fs.exists("/a/b/hello.txt"));

        fs.rename("/a/b/hello.txt", "/a/b2/hello.txt")?;
        assert!(!fs.exists("/a/b/hello.txt"));
        assert!(fs.exists("/a/b2/hello.txt"));

        fs.rename("/a/b2", "/a/b3")?;
        assert!(!fs.exists("/a/b/hello.txt"));
        assert!(!fs.exists("/a/b2/hello.txt"));
        assert!(fs.exists("/a/b3/hello.txt"));

        Ok(())
    }

    #[test]
    fn base_err() -> Result<()> {
        let fs = MemoryFileSystem::default();
        assert!(fs.read("/does/not/exist").is_err());
        Ok(())
    }
}
