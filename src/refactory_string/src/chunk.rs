use crate::error::Error;

/// Chunks are parts of a memory that have an intro and an outro.
/// They are chunks of bytes, and not strings, as we export two types; a
/// string that deals with String content, and a buffer that deals with
/// binary data. Because both reuse the same chunk type (this one), this type
/// is storage agnostic.
pub(crate) struct Chunk<'a> {
    pub left: Option<Vec<u8>>,
    pub right: Option<Vec<u8>>,
    pub content: Option<&'a [u8]>,
    pub start: usize,
    pub end: usize,
}

impl<'a> Chunk<'a> {
    pub fn new(original_content: &'a [u8]) -> Chunk<'a> {
        Chunk {
            left: Some(Vec::new()),
            content: Some(&original_content[..]),
            right: Some(Vec::new()),
            start: 0,
            end: original_content.len(),
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        if let Some(ref l) = self.left {
            v.extend_from_slice(l);
        }
        if let Some(ref c) = self.content {
            v.extend_from_slice(c);
        }
        if let Some(ref r) = self.right {
            v.extend_from_slice(r);
        }
        v
    }

    pub fn append_right(&mut self, content: &[u8]) -> Result<(), Error> {
        if let Some(ref mut r) = self.right {
            r.extend_from_slice(content);
        }
        Ok(())
    }

    pub fn append_left(&mut self, content: &[u8]) -> Result<(), Error> {
        if let Some(ref mut l) = self.left {
            l.extend_from_slice(content);
        }
        Ok(())
    }

    pub fn prepend_right(&mut self, content: &[u8]) -> Result<(), Error> {
        if let Some(ref mut r) = self.right {
            let mut tmp = content.to_owned();
            tmp.extend_from_slice(r);
            self.right = Some(tmp);
        }
        Ok(())
    }

    pub fn prepend_left(&mut self, content: &[u8]) -> Result<(), Error> {
        if let Some(ref mut l) = self.left {
            let mut tmp = content.to_owned();
            tmp.extend_from_slice(l);
            self.left = Some(tmp);
        }
        Ok(())
    }
}
