use crate::chunk::Chunk;
use crate::error::Error;
use alloc::boxed::Box;
use alloc::string::String;

pub(crate) struct ChunkList<'a> {
    head: Link<'a>,
}

type Link<'a> = Option<Box<Node<'a>>>;
struct Node<'a> {
    pub elem: Chunk<'a>,
    pub next: Link<'a>,
}

impl<'a> ChunkList<'a> {
    pub fn new(original_content: &'a str) -> Self {
        ChunkList {
            head: Some(Box::new(Node {
                elem: Chunk::new(original_content),
                next: None,
            })),
        }
    }

    pub fn split(&mut self, index: usize) -> Result<(&mut Chunk<'a>, &mut Chunk<'a>), Error> {
        let (prev, curr) = self.get_node_at(index);

        // We found a node that matches.
        if let Some(node) = curr {
            let chunk = &mut node.elem;

            if index == chunk.start && prev.is_some() {
                return Ok((&mut prev.unwrap().elem, chunk));
            }
            if index == chunk.end && node.next.is_some() {
                return Ok((chunk, &mut node.next.as_mut().unwrap().as_mut().elem));
            }

            let inner_start = index - chunk.start;
            let orig_right = chunk.right.as_ref().map(|_| String::new());
            let right = chunk.right.take();
            let new_chunk = Chunk {
                // Left is None if Chunk does not have a right.
                left: right.as_ref().map(|_| String::new()),
                right,
                content: chunk.content.as_ref().map(|c| &c[inner_start..]),
                start: index,
                end: chunk.end,
            };
            let box_node = Box::new(Node {
                elem: new_chunk,
                next: node.next.take(),
            });

            chunk.content = chunk.content.as_ref().map(|x| &x[..inner_start]);
            chunk.end = index;
            chunk.right = orig_right;
            node.next = Some(box_node);

            Ok((chunk, &mut node.next.as_mut().unwrap().as_mut().elem))
        } else {
            Err(Error::IndexOutOfBoundError(index))
        }
    }

    pub fn remove(&mut self, start: usize, end: usize) -> Result<(), Error> {
        if start >= end {
            return Ok(());
        }
        let _ = self.split(start)?;
        let _ = self.split(end)?;

        let mut it = self.iter_mut();
        while let Some(c) = it.next() {
            if c.start >= start && start < c.end {
                c.content = None;
                c.left = None;
                c.right = None;
            }
            if c.end >= end {
                break; // No need to continue.
            }
        }

        Ok(())
    }

    fn get_node_at(&mut self, index: usize) -> (Option<&mut Node<'a>>, Option<&mut Node<'a>>) {
        let mut current = &mut self.head;
        let mut previous: Option<&mut Node<'a>> = None;
        while let Some(ref mut box_node) = current {
            let node = box_node.as_mut();
            if index >= node.elem.start && index <= node.elem.end {
                return (previous, Some(node));
            }
            unsafe {
                let node_ptr = node as *mut Node<'a>;
                let next_ptr = &mut node.next as *mut Link<'a>;
                previous = Some(&mut *node_ptr);
                current = &mut *next_ptr;
            }
        }
        (previous, None)
    }

    #[cfg(test)]
    pub fn get_chunk_at(&self, index: usize) -> Option<&Chunk<'a>> {
        let mut current = &self.head;
        while let Some(ref box_node) = current {
            let node = box_node.as_ref();
            if index >= node.elem.start && index < node.elem.end {
                return Some(&node.elem);
            }
            current = &node.next;
        }
        None
    }

    #[cfg(test)]
    pub fn get_mut_chunk_at(&mut self, index: usize) -> Option<&mut Chunk<'a>> {
        self.get_node_at(index).1.map(|node| &mut node.elem)
    }

    pub fn iter<'b: 'a>(&'b self) -> Iter<'_, 'a> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, 'a> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<'a> Drop for ChunkList<'a> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub(crate) struct Iter<'b, 'a: 'b> {
    next: Option<&'a Node<'b>>,
}

impl<'b, 'a: 'b> Iterator for Iter<'b, 'a> {
    type Item = &'b Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub(crate) struct IterMut<'b, 'a: 'b> {
    next: Option<&'b mut Node<'a>>,
}

impl<'b, 'a: 'b> Iterator for IterMut<'b, 'a> {
    type Item = &'b mut Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}
