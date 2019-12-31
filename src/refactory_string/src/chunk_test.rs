#![cfg(test)]
use crate::chunk::Chunk;
use crate::chunk_list::ChunkList;
use crate::error::Error;

#[test]
fn basic() -> Result<(), Error> {
    let content: Vec<u8> = vec![1, 2, 3, 4];
    let mut cl = ChunkList::new(&content);

    if let Some(c) = cl.get_chunk_at(0) {
        assert_eq!(c.start, 0);
        assert_eq!(c.end, 4);
        assert_eq!(c.to_bytes(), content);
    } else {
        assert!(false);
    }

    let (c1, c2) = cl.split(2)?;
    assert_eq!(c1.to_bytes().as_slice(), &content[..2]);
    assert_eq!(c2.to_bytes().as_slice(), &content[2..]);

    // Verify that slicing twice at the same index returns the same chunks.
    unsafe {
        let clptr = &mut cl as *mut ChunkList;
        let (c1, c2) = (*clptr).split(0)?;
        let (c1_, c2_) = (*clptr).split(0)?;
        assert_eq!(c1 as *const Chunk, c1_ as *const Chunk);
        assert_eq!(c2 as *const Chunk, c2_ as *const Chunk);
    }

    unsafe {
        let clptr = &mut cl as *mut ChunkList;
        let (c1, c2) = (*clptr).split(3)?;
        let (c1_, c2_) = (*clptr).split(3)?;
        assert_eq!(c1 as *const Chunk, c1_ as *const Chunk);
        assert_eq!(c2 as *const Chunk, c2_ as *const Chunk);
    }

    unsafe {
        let clptr = &mut cl as *mut ChunkList;
        let (c1, c2) = (*clptr).split(4)?;
        let (c1_, c2_) = (*clptr).split(4)?;
        assert_eq!(c1 as *const Chunk, c1_ as *const Chunk);
        assert_eq!(c2 as *const Chunk, c2_ as *const Chunk);
    }

    Ok(())
}

#[test]
fn append() -> Result<(), Error> {
    let content: Vec<u8> = vec![1, 2, 3, 4];
    let mut cl = ChunkList::new(&content);

    if let Some(ref mut c) = cl.get_mut_chunk_at(0).take() {
        c.append_right(&[5, 6, 7, 8])?;
        assert_eq!(c.to_bytes().as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8]);
    }

    let _ = cl.split(2)?;
    let mut it = cl.iter();
    assert_eq!(it.next().map(|x| x.to_bytes()), Some(vec![1, 2]));
    assert_eq!(
        it.next().map(|x| x.to_bytes()),
        Some(vec![3, 4, 5, 6, 7, 8,])
    );
    assert_eq!(it.next().map(|x| x.to_bytes()), None);

    Ok(())
}

#[test]
fn prepend() -> Result<(), Error> {
    let content: Vec<u8> = vec![1, 2, 3, 4];
    let mut cl = ChunkList::new(&content);

    if let Some(ref mut c) = cl.get_mut_chunk_at(0).take() {
        c.prepend_left(&[5, 6, 7, 8])?;
        assert_eq!(c.to_bytes().as_slice(), &[5, 6, 7, 8, 1, 2, 3, 4]);
    }

    let _ = cl.split(2)?;
    let mut it = cl.iter();
    assert_eq!(
        it.next().map(|x| x.to_bytes()),
        Some(vec![5, 6, 7, 8, 1, 2])
    );
    assert_eq!(it.next().map(|x| x.to_bytes()), Some(vec![3, 4]));
    assert_eq!(it.next().map(|x| x.to_bytes()), None);

    Ok(())
}

#[test]
fn append_prepend() -> Result<(), Error> {
    let content: Vec<u8> = vec![2, 3];
    let mut cl = ChunkList::new(&content);

    let chunk = cl.get_mut_chunk_at(0).ok_or(Error::InvalidInternalState)?;
    chunk.append_left(&[1])?;
    chunk.prepend_left(&[0])?;
    chunk.append_left(&[8])?;
    chunk.prepend_left(&[9])?;

    assert_eq!(&chunk.to_bytes(), &[9, 0, 1, 8, 2, 3]);

    chunk.append_right(&[4])?;
    chunk.prepend_right(&[5])?;
    chunk.append_right(&[6])?;
    chunk.prepend_right(&[7])?;
    assert_eq!(&chunk.to_bytes(), &[9, 0, 1, 8, 2, 3, 7, 5, 4, 6]);

    Ok(())
}

#[test]
fn slice() -> Result<(), Error> {
    let content = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut cl = ChunkList::new(&content);

    let _ = cl.split(1)?;
    let _ = cl.split(3)?;
    let _ = cl.split(5)?;
    let _ = cl.split(3)?;
    let _ = cl.split(4)?;
    let _ = cl.split(1)?;
    let _ = cl.split(4)?;
    let _ = cl.split(5)?;
    let _ = cl.split(8)?;

    assert_eq!(cl.iter().count(), 6);

    let _ = cl.split(7)?;
    assert_eq!(cl.iter().count(), 7);

    let _ = cl.split(7)?;
    let _ = cl.split(3)?;
    let _ = cl.split(4)?;
    assert_eq!(cl.iter().count(), 7);
    Ok(())
}

#[test]
fn get_chunk_at() -> Result<(), Error> {
    let content: Vec<u8> = vec![1, 2, 3, 4];
    let mut cl = ChunkList::new(&content);

    let _ = cl.split(2)?;
    assert_eq!(cl.get_chunk_at(0).map(|x| x.to_bytes()), Some(vec![1, 2]));
    assert_eq!(cl.get_chunk_at(1).map(|x| x.to_bytes()), Some(vec![1, 2]));
    assert_eq!(cl.get_chunk_at(2).map(|x| x.to_bytes()), Some(vec![3, 4]));
    assert_eq!(cl.get_chunk_at(3).map(|x| x.to_bytes()), Some(vec![3, 4]));
    assert_eq!(cl.get_chunk_at(4).map(|x| x.to_bytes()), None);

    Ok(())
}
