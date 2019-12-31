extern crate alloc;

mod chunk;
mod chunk_list;
mod error;
mod refactory_string;

pub use crate::refactory_string::*;

// Tests
mod chunk_test;
mod refactory_string_own_test;
mod refactory_string_test;
