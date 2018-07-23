//! # A wrapper for libxslt
//! This library provides an interface to a subset of the libxslt API.
//!
#![deny(missing_docs)]
extern crate libc;
extern crate libxml;

// Bindings to the C interface
mod bindings;
/// Parser for loading an XSLT stylesheet from a file or string source.
pub mod parser;
/// Stylesheet functionality for Document transformation.
pub mod stylesheet;
