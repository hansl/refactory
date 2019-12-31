#![cfg(test)]
use crate::error::Error;
use crate::RefactoryString;

#[test]
fn basic() -> Result<(), Error> {
    //                   01234567890
    let content = "Hello World";
    let mut buffer = RefactoryString::new(&content);

    buffer.append_right(6, "Beautiful ")?;
    assert_eq!(&buffer.to_string()?, "Hello Beautiful World");

    buffer.append_right(6, "Great ")?;
    assert_eq!(&buffer.to_string()?, "Hello Beautiful Great World");

    buffer.append_right(0, "1 ")?;
    assert_eq!(&buffer.to_string()?, "1 Hello Beautiful Great World");

    buffer.append_right(5, "2 ")?;
    assert_eq!(&buffer.to_string()?, "1 Hello2  Beautiful Great World");

    buffer.append_right(8, "3 ")?;
    assert_eq!(&buffer.to_string()?, "1 Hello2  Beautiful Great Wo3 rld");

    buffer.append_right(0, "4 ")?;
    assert_eq!(&buffer.to_string()?, "1 4 Hello2  Beautiful Great Wo3 rld");

    buffer.append_right(8, "5 ")?;
    assert_eq!(
        &buffer.to_string()?,
        "1 4 Hello2  Beautiful Great Wo3 5 rld"
    );

    buffer.append_right(1, "a ")?;
    assert_eq!(
        &buffer.to_string()?,
        "1 4 Ha ello2  Beautiful Great Wo3 5 rld"
    );

    buffer.append_right(2, "b ")?;
    assert_eq!(
        &buffer.to_string()?,
        "1 4 Ha eb llo2  Beautiful Great Wo3 5 rld"
    );

    buffer.append_right(7, "c ")?;
    assert_eq!(
        &buffer.to_string()?,
        "1 4 Ha eb llo2  Beautiful Great Wc o3 5 rld"
    );

    buffer.append_right(11, "d 6")?;
    assert_eq!(
        &buffer.to_string()?,
        "1 4 Ha eb llo2  Beautiful Great Wc o3 5 rldd 6"
    );

    Ok(())
}

#[test]
fn magic_string_example() -> Result<(), Error> {
    let mut s = RefactoryString::new("problems = 99");

    s.overwrite(0, 8, "answer")?;
    assert_eq!(&s.to_string()?, "answer = 99");

    s.overwrite(11, 13, "42")?;
    assert_eq!(&s.to_string()?, "answer = 42");

    s.prepend("var ")?;
    s.append(";")?;
    assert_eq!(&s.to_string()?, "var answer = 42;");

    Ok(())
}

#[test]
fn readme_example() -> Result<(), Error> {
    let example = String::from(r#"let i = 1;\nprintln!("{}", i + 5);"#);
    let mut rs = RefactoryString::new(&example);

    rs.overwrite(4,5,"new_var_name")?;
    rs.overwrite(27,28,"new_var_name")?;

    assert_eq!(&rs.to_string()?, r#"let new_var_name = 1;\nprintln!("{}", new_var_name + 5);"#);
    Ok(())
}
