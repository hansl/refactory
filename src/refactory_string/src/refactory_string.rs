use crate::error::Error;
use crate::chunk_list::ChunkList;
use alloc::string::String;

/// A RefactoryBuffer specialization that only accepts and returns UTF-8 strings. This is
/// what should be used when modifying a source string/file content. It uses RefactoryBuffer
/// and converts everything conveniently.
pub struct RefactoryString<'a> {
    chunks: ChunkList<'a>
}

impl<'a> RefactoryString<'a> {
    /// Create a new RefactoryString from the content. Never owns the original content, but
    /// owns every changes made to it.
    pub fn new(content: &'a str) -> RefactoryString<'a> {
        RefactoryString {
            chunks:ChunkList::new(content)
        }
    }

    /// The original length of the content it contains.
    pub fn len(&self) -> usize {
        self.chunks.iter().fold(0, |a,x| a + x.len())
    }

    /// Serialize the changes to a string.
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for it in self.chunks.iter() {
            s.push_str(&it.to_string());
        }
        s
    }

    #[inline]
    fn do_insert(
        &mut self,
        index: usize,
        content: &str,
        left: bool,
        append: bool,
    ) -> Result<(), Error> {
        let (l, r) = self.chunks.split(index)?;

        if append {
            if left {
                l.append_right(content)
            } else {
                r.append_left(content)
            }
        } else {
            if left {
                l.prepend_right(content)
            } else {
                r.prepend_left(content)
            }
        }
    }

    /// Append the content to the left of the index.
    pub fn append_left(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.do_insert(index, content, true, true)
    }

    /// Prepend the content to the left of the index.
    pub fn prepend_left(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.do_insert(index, content, true, false)
    }

    /// Append the content to the right of the index.
    pub fn append_right(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.do_insert(index, content, false, true)
    }

    /// Prepend the content to the right of the index.
    pub fn prepend_right(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.do_insert(index, content, false,false)
    }

    /// Prepend the content to the whole RefactoryString.
    pub fn prepend(&mut self, content: &str) -> Result<(), Error> {
        self.prepend_left(0, content)
    }

    /// Append the content to the whole RefactoryString.
    pub fn append(&mut self, content: &str) -> Result<(), Error> {
        self.append_right(self.len(), content)
    }

    /// Overwrite the content at the indices [start, end].
    pub fn overwrite(&mut self, start: usize, end: usize, content: &str) -> Result<(), Error> {
        self.remove(start, end)?;
        self.append_left(start, content)?;
        Ok(())
    }

    /// Remove the content between two indices.
    pub fn remove(&mut self, start: usize, end: usize) -> Result<(), Error> {
        self.chunks.remove(start, end)
    }
}
