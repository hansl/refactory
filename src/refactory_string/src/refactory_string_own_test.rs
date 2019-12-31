// These tests are meant to be an exact copy of the tests on MagicString at
// https://github.com/Rich-Harris/magic-string/blob/master/test/MagicString.js
// We are skipping all tests that verify it returns this, as we don't.
#![cfg(test)]
use crate::error::Error;
use crate::RefactoryString;

#[test]
fn append() -> Result<(), Error> {
    let mut s = RefactoryString::new("abcdefghijkl");

    s.append("xyz")?;
    assert_eq!(&s.to_string()?, "abcdefghijklxyz");

    s.append("uvw")?;
    assert_eq!(&s.to_string()?, "abcdefghijklxyzuvw");

    Ok(())
}

#[test]
fn prepend() -> Result<(), Error> {
    let mut s = RefactoryString::new("abcdefghijkl");

    s.prepend("xyz")?;
    assert_eq!(&s.to_string()?, "xyzabcdefghijkl");

    s.prepend("uvw")?;
    assert_eq!(&s.to_string()?, "uvwxyzabcdefghijkl");

    Ok(())
}

#[test]
fn preserves_intended_order() -> Result<(), Error> {
    let mut s = RefactoryString::new("0123456789");

    s.append_left(5, "A")?;
    s.prepend_right(5, "a")?;
    s.prepend_right(5, "b")?;
    s.append_left(5, "B")?;
    s.append_left(5, "C")?;
    s.prepend_right(5, "c")?;

    assert_eq!(&s.to_string()?, "01234ABCcba56789");

    s.prepend_left(5, "<")?;
    s.prepend_left(5, "{")?;
    assert_eq!(&s.to_string()?, "01234{<ABCcba56789");

    s.append_right(5, ">")?;
    s.append_right(5, "}")?;
    assert_eq!(&s.to_string()?, "01234{<ABCcba>}56789");

    s.append_left(5, "(")?;
    s.append_left(5, "[")?;
    assert_eq!(&s.to_string()?, "01234{<ABC([cba>}56789");

    s.prepend_right(5, ")")?;
    s.prepend_right(5, "]")?;
    assert_eq!(&s.to_string()?, "01234{<ABC([])cba>}56789");

    Ok(())
}

#[test]
fn preserves_intended_order_at_beginning() -> Result<(), Error> {
    let mut s = RefactoryString::new("x");

    s.append_left(0, "1")?;
    s.prepend_left(0, "2")?;
    s.append_left(0, "3")?;
    s.prepend_left(0, "4")?;

    assert_eq!(&s.to_string()?, "4213x");

    Ok(())
}

#[test]
fn preserves_intended_order_at_end() -> Result<(), Error> {
    let mut s = RefactoryString::new("x");

    s.append_right(1, "1")?;
    s.prepend_right(1, "2")?;
    s.append_right(1, "3")?;
    s.prepend_right(1, "4")?;

    assert_eq!(&s.to_string()?, "x4213");

    Ok(())
}
