#![doc = include_str!("../README.md")]

/// The `mapper` module provides functionality for memory mapping files into memor
pub mod mapper;

/// The `data` module defines data structures and constants used in minidump parsing.
pub mod data;

/// The `error` module defines error types used throughout the library.
pub mod error;

/// The `parse` module contains the core logic for parsing minidump files.
pub mod parse;
pub use parse::*;
