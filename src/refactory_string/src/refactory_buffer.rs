use crate::chunk_list::ChunkList;
use crate::error::Error;

pub struct RefactoryBuffer<'a> {
    pub(crate) chunks: ChunkList<'a>,
}

impl<'a> RefactoryBuffer<'a> {
    pub fn new(content: &'a [u8]) -> RefactoryBuffer<'a> {
        RefactoryBuffer {
            chunks: ChunkList::new(content),
        }
    }

    pub fn len(&self) -> usize {
        self.chunks.iter().fold(0, |a, c| a + c.len())
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut v = Vec::new();
        for it in self.chunks.iter() {
            v.append(&mut it.to_bytes());
        }
        Ok(v)
    }

    #[inline]
    fn do_insert(
        &mut self,
        index: usize,
        content: &[u8],
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

    pub fn append_left(&mut self, index: usize, content: &[u8]) -> Result<(), Error> {
        self.do_insert(index, content, true, true)
    }
    pub fn prepend_left(&mut self, index: usize, content: &[u8]) -> Result<(), Error> {
        self.do_insert(index, content, true, false)
    }
    pub fn append_right(&mut self, index: usize, content: &[u8]) -> Result<(), Error> {
        self.do_insert(index, content, false, true)
    }
    pub fn prepend_right(&mut self, index: usize, content: &[u8]) -> Result<(), Error> {
        self.do_insert(index, content, false, false)
    }
    pub fn prepend(&mut self, content: &[u8]) -> Result<(), Error> {
        self.prepend_left(0, content)
    }
    pub fn append(&mut self, content: &[u8]) -> Result<(), Error> {
        self.append_right(self.len(), content)
    }

    pub fn remove(&mut self, start: usize, end: usize) -> Result<(), Error> {
        self.chunks.remove(start, end)
    }
}
