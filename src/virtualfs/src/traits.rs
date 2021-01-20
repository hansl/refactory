use crate::error::Result;
use crate::path::{OwnedPath, Path};

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait Write {
    fn write(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub struct File<'a, Fs: FileSystem + Sized> {
    fs: &'a Fs,
    offset: usize,
    path: OwnedPath,
}

impl<'a, Fs: FileSystem> File<'a, Fs> {
    pub fn create(fs: &'a Fs, path: &Path) -> Self {
        File {
            fs,
            offset: 0,
            path: path.to_owned(),
        }
    }
}

impl<'a, Fs: FileSystem> Read for File<'a, Fs> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.fs.read(self.path.as_ref(), buf)?;
        self.offset += n;
        Ok(n)
    }
}

pub trait ReadOnlyFileSystem: Send + Sized + 'static {
    type Fs = Self;

    fn read(&self, path: &Path, offset: usize, buf: &mut [u8]) -> Result<usize>;

    fn create(&self, path: &Path) -> Result<File<'_, Self>>;
    fn open(&self, path: &Path) -> Result<File<'_, Self>>;
    fn delete(&self, path: &Path) -> Result<()>;
    fn rename(&self, from: &Path, to: &Path) -> Result<()>;
}

pub trait FileSystem: ReadOnlyFileSystem {
    type Fs = Self;

    fn write(&self, path: &Path, offset: usize, buf: &[u8]) -> Result<usize>;
}
