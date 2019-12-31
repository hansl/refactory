use crate::error::Error;
use crate::RefactoryBuffer;

/// A RefactoryBuffer specialization that only accepts and returns UTF-8 strings. This is
/// what should be used when modifying a source string/file content. It uses RefactoryBuffer
/// and converts everything conveniently.
pub struct RefactoryString<'a>(RefactoryBuffer<'a>);

impl<'a> RefactoryString<'a> {
    /// Create a new RefactoryString from the content. Never owns the original content, but
    /// owns every changes made to it.
    pub fn new(content: &'a str) -> RefactoryString<'a> {
        RefactoryString(RefactoryBuffer::new(content.as_bytes()))
    }

    /// The original length of the content it contains.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Serialize the changes to a string.
    pub fn to_string(&self) -> Result<String, Error> {
        return String::from_utf8(self.0.to_bytes()?).map_err(|_| Error::InvalidInternalState);
    }

    /// Append the content to the left of the index.
    pub fn append_left(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.0.append_left(index, content.as_bytes())
    }

    /// Prepend the content to the left of the index.
    pub fn prepend_left(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.0.prepend_left(index, content.as_bytes())
    }

    /// Append the content to the right of the index.
    pub fn append_right(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.0.append_right(index, content.as_bytes())
    }

    /// Prepend the content to the right of the index.
    pub fn prepend_right(&mut self, index: usize, content: &str) -> Result<(), Error> {
        self.0.prepend_right(index, content.as_bytes())
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
        self.0.remove(start, end)
    }
}
