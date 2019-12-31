#![cfg(test)]
use crate::chunk::Chunk;
use crate::chunk_list::ChunkList;
use crate::error::Error;
use alloc::string::ToString;

#[test]
fn basic() -> Result<(), Error> {
    let content = "1234";
    let mut cl = ChunkList::new(&content);

    if let Some(c) = cl.get_chunk_at(0) {
        assert_eq!(c.start, 0);
        assert_eq!(c.end, 4);
        assert_eq!(&c.to_string(), content);
    } else {
        assert!(false);
    }

    let (c1, c2) = cl.split(2)?;
    assert_eq!(&c1.to_string(), &content[..2]);
    assert_eq!(&c2.to_string(), &content[2..]);

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
    let mut cl = ChunkList::new("1234");

    if let Some(ref mut c) = cl.get_mut_chunk_at(0).take() {
        c.append_right("5678")?;
        assert_eq!(&c.to_string(), "12345678");
    }

    let _ = cl.split(2)?;
    let mut it = cl.iter();
    assert_eq!(it.next().map(|x| x.to_string()), Some("12".to_string()));
    assert_eq!(it.next().map(|x| x.to_string()), Some("345678".to_string()));
    assert_eq!(it.next().map(|x| x.to_string()), None);

    Ok(())
}

#[test]
fn prepend() -> Result<(), Error> {
    let mut cl = ChunkList::new("1234");

    if let Some(ref mut c) = cl.get_mut_chunk_at(0).take() {
        c.prepend_left("5678")?;
        assert_eq!(&c.to_string(), "56781234");
    }

    let _ = cl.split(2)?;
    let mut it = cl.iter();
    assert_eq!(it.next().map(|x| x.to_string()), Some("567812".to_string()));
    assert_eq!(it.next().map(|x| x.to_string()), Some("34".to_string()));
    assert_eq!(it.next().map(|x| x.to_string()), None);

    Ok(())
}

#[test]
fn append_prepend() -> Result<(), Error> {
    let mut cl = ChunkList::new("23");

    let chunk = cl.get_mut_chunk_at(0).ok_or(Error::InvalidInternalState)?;
    chunk.append_left("1")?;
    chunk.prepend_left("0")?;
    chunk.append_left("8")?;
    chunk.prepend_left("9")?;

    assert_eq!(&chunk.to_string(), "901823");

    chunk.append_right("4")?;
    chunk.prepend_right("5")?;
    chunk.append_right("6")?;
    chunk.prepend_right("7")?;
    assert_eq!(&chunk.to_string(), "9018237546");

    Ok(())
}

#[test]
fn slice() -> Result<(), Error> {
    let mut cl = ChunkList::new("012345678");

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
    let mut cl = ChunkList::new("1234");

    let _ = cl.split(2)?;
    assert_eq!(cl.get_chunk_at(0).map(|x| x.to_string()), Some("12".to_string()));
    assert_eq!(cl.get_chunk_at(1).map(|x| x.to_string()), Some("12".to_string()));
    assert_eq!(cl.get_chunk_at(2).map(|x| x.to_string()), Some("34".to_string()));
    assert_eq!(cl.get_chunk_at(3).map(|x| x.to_string()), Some("34".to_string()));
    assert_eq!(cl.get_chunk_at(4).map(|x| x.to_string()), None);

    Ok(())
}
