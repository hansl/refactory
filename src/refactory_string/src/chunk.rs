use crate::error::Error;
use alloc::borrow::ToOwned;
use alloc::string::String;

/// Chunks are parts of a memory that have an intro and an outro.
/// They are chunks of bytes, and not strings, as we export two types; a
/// string that deals with String content, and a buffer that deals with
/// binary data. Because both reuse the same chunk type (this one), this type
/// is storage agnostic.
pub(crate) struct Chunk<'a> {
    pub left: Option<String>,
    pub right: Option<String>,
    pub content: Option<&'a str>,
    pub start: usize,
    pub end: usize,
}

impl<'a> Chunk<'a> {
    pub fn new(original_content: &'a str) -> Chunk<'a> {
        Chunk {
            left: Some(String::new()),
            content: Some(&original_content[..]),
            right: Some(String::new()),
            start: 0,
            end: original_content.len(),
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        if let Some(ref l) = self.left {
            s.push_str(l);
        }
        if let Some(ref c) = self.content {
            s.push_str(c);
        }
        if let Some(ref r) = self.right {
            s.push_str(r);
        }
        s
    }

    pub fn append_right(&mut self, content: &str) -> Result<(), Error> {
        if let Some(ref mut r) = self.right {
            r.push_str(content);
        }
        Ok(())
    }

    pub fn append_left(&mut self, content: &str) -> Result<(), Error> {
        if let Some(ref mut l) = self.left {
            l.push_str(content);
        }
        Ok(())
    }

    pub fn prepend_right(&mut self, content: &str) -> Result<(), Error> {
        if let Some(ref mut r) = self.right {
            let mut tmp = content.to_owned();
            tmp.push_str(r);
            self.right = Some(tmp);
        }
        Ok(())
    }

    pub fn prepend_left(&mut self, content: &str) -> Result<(), Error> {
        if let Some(ref mut l) = self.left {
            let mut tmp = content.to_owned();
            tmp.push_str(l);
            self.left = Some(tmp);
        }
        Ok(())
    }
}
