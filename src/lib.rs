#![doc = include_str!("../readme.md")]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
#![deny(clippy::default_numeric_fallback)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod ignore;
pub use ignore::*;
mod keep;
pub use keep::*;
#[cfg(feature = "stack")]
mod stack;
#[cfg(feature = "stack")]
pub use stack::*;
#[cfg(feature = "tree")]
mod tree;
#[cfg(feature = "tree")]
pub use tree::*;

/// Used to determine by Keep and Stack to determine their Behavior
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct First;

/// Used to determine by Keep and Stack to determine their Behavior
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Last;
