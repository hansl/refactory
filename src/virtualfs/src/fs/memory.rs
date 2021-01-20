use crate::error::Result;
use crate::path::Path;
use crate::traits::{File, FileSystem};

struct MemoryFileSystem {}

impl MemoryFileSystem {}

struct MemoryFile {}
impl MemoryFile {}

impl FileSystem for MemoryFileSystem {
    fn create(&self, path: &Path) -> Result<File<'_, Self>> {}
    fn open(&self, path: &Path) -> Result<File<'_, Self>> {}
    fn delete(&self, path: &Path) -> Result<()>;
    fn rename(&self, from: &Path, to: &Path) -> Result<()>;
}
