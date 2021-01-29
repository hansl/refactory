use crate::error::Result;
use crate::path::{OwnedPath, Path};

mod memory;

pub struct Entry<'a, Fs: FileSystem + Sized> {
    fs: &'a Fs,
    offset: usize,
    path: OwnedPath,
}

impl<'a, Fs: FileSystem> Entry<'a, Fs> {
    pub fn new<P: AsRef<Path>>(fs: &'a Fs, path: P) -> Self {
        Entry {
            fs,
            offset: 0,
            path: path.as_ref().to_owned(),
        }
    }
}

impl<'a, Fs: FileSystem> std::io::Read for Entry<'a, Fs> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes: &[u8] = self
            .fs
            .read(&self.path)
            .map_err(std::convert::Into::<std::io::Error>::into)?;
        let len = std::cmp::min(bytes.len(), buf.len());
        buf[0..len].copy_from_slice(&bytes[0..len]);
        self.offset += len;
        Ok(len)
    }
}

// pub struct Stat<'a, Fs: FileSystem + Sized> {
//     fs: &'a Fs,
//     path: OwnedPath,
// }

pub trait FileSystem: Send + Sized {
    fn read<P: AsRef<Path>>(&self, path: P) -> Result<&[u8]>;
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool;
    // fn stat<P: AsRef<Path>>(&self, path: P) -> Result<Stat<'_, Self>>;

    fn write<P: AsRef<Path>, C: AsRef<[u8]>>(&mut self, path: P, content: C) -> Result<()>;
    fn delete<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
    fn rename<From: AsRef<Path>, To: AsRef<Path>>(&mut self, from: From, to: To) -> Result<()>;

    fn entry<P: AsRef<Path>>(&self, path: P) -> Result<Entry<'_, Self>> {
        Ok(Entry::new(self, path))
    }
}
