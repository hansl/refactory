# RefactoryString
A library to modify a string using original indices, inspired  by Rich Harris' MagicString (see [here](https://github.com/Rich-Harris/magic-string)).

Suppose you have some source code and you want to modify it. If the source code that
you're using doesn't have a lossless AST parser _and_ writer, you won't be able to
parse it, update it, then save it back. This is where RefactoryString comes in handy; it
allows you to modify a text content using its original indices. It is also very fast.

For example, you may want to replace the variable name `i` in the following code:

```
let i = 1;
println!("{}", i + 5);
```

One struggle is to do the transformation in order, and you need to reparse the AST
everytime you add something new. With `RefactoryString` you don't need to worry
about it; just `overwrite`, append or prepend to the left or right of indices in
the original string, and serialize to string;

```rust
fn do_it() -> Result<(), refactory_string::Error> {
    let example = String::from(r#"let i = 1;\nprintln!("{}", i + 5);"#);
    let mut rs = RefactoryString::new(&example);

    rs.overwrite(4,5,"new_var_name")?;
    rs.overwrite(27,28,"new_var_name")?;

    assert_eq!(&rs.to_string()?, r#"let new_var_name = 1;\nprintln!("{}", new_var_name + 5);"#);
    Ok(())
}
```

## Documentation


## RefactoryBuffer
The `RefactoryString` type deals with strings, but this crate also export a type
that can deal with binary data; `RefactoryBuffer`. This is the same structure but
replaces `to_string()` with `to_bytes()`, which returns a `Vec<u8>`.

# TODO
See our issue list [here]().
