#![cfg(test)]

use crate::error::Error;
use crate::RefactoryBuffer;

#[test]
fn basic() -> Result<(), Error> {
    let content = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut buffer = RefactoryBuffer::new(&content);

    buffer.prepend_left(3, &[10])?;
    assert_eq!(buffer.to_bytes()?, vec![0, 1, 2, 10, 3, 4, 5, 6, 7, 8]);

    buffer.append_right(3, &[11])?;
    assert_eq!(buffer.to_bytes()?, vec![0, 1, 2, 10, 11, 3, 4, 5, 6, 7, 8]);

    Ok(())
}

#[test]
fn remove() -> Result<(), Error> {
    let content = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut buffer = RefactoryBuffer::new(&content);

    buffer.remove(1, 3)?;
    assert_eq!(buffer.to_bytes()?, vec![0, 3, 4, 5, 6, 7, 8]);

    buffer.remove(3, 6)?;
    assert_eq!(buffer.to_bytes()?, vec![0, 6, 7, 8]);

    buffer.remove(4, 5)?;
    assert_eq!(buffer.to_bytes()?, vec![0, 6, 7, 8]);

    buffer.append_left(5, &[10])?;
    buffer.append_right(5, &[11])?;
    buffer.prepend_left(5, &[12])?;
    buffer.prepend_right(5, &[13])?;
    assert_eq!(buffer.to_bytes()?, vec![0, 6, 7, 8]);

    Ok(())
}
