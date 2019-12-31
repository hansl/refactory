# RefactoryString
A library to modify a string using original indices, inspired by Rich
  Harris' MagicString (see [here](https://github.com/Rich-Harris/magic-string)).

[![Crates.io](https://img.shields.io/crates/v/refactory_string.svg)
![Downloads](https://img.shields.io/crates/d/refactory_string.svg)](https://crates.io/crates/refactory_string)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/hansl/refactory/test?label=test)](https://github.com/hansl/refactory/actions?query=workflow%3Atest)
[![docs.rs](https://docs.rs/refactory_string/badge.svg)](https://docs.rs/refactory_string)
![rustc ^1.38.0](https://img.shields.io/badge/rustc-^1.38.0-blue.svg)

<a href="https://www.buymeacoffee.com/hansl" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="42px" style="border-radius: 5px"></a> 

----

Suppose you have some source code and you want to modify it. If the source code that
  you're using doesn't have a lossless AST parser _and_ writer, you won't be able to
  parse it, update it, then save it back. This is where RefactoryString comes in
  handy; it allows you to modify a text content using its original indices. It is
  also very fast.

For example, you may want to replace the variable name `i` with `new_var_name` in the following code:

```
let i = 1;
println!("{}", i + 5);
```

One struggle is to do the transformation in the appropriate order (so you have
  to queue all changes), and you need to reparse the AST everytime you add something
  new. With `RefactoryString` you don't need to worry about it; just `overwrite`,
  append or prepend to the left or right of indices in the original string, and
  serialize to string;

```rust
fn do_it() -> Result<(), refactory_string::Error> {
    let example = String::from(r#"let i = 1;\nprintln!("{}", i + 5);"#);
    let mut rs = RefactoryString::new(&example);

    rs.overwrite(4, 5, "new_var_name")?;
    rs.overwrite(27, 28, "new_var_name")?;  // Using indices in the original content.

    assert_eq!(&rs.to_string()?, r#"let new_var_name = 1;\nprintln!("{}", new_var_name + 5);"#);
    Ok(())
}
```

## Documentation
Documentation can be found [here](https://docs.rs/refactory_string) and is always
  improving.

## TODO
See our issue list [here](https://github.com/hansl/refactory/labels/pkg%3Arefactory_string).
