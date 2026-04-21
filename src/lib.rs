//! # A wrapper for libxslt
//! This library provides an interface to a subset of the libxslt API.
//!
#![deny(missing_docs)]
extern crate libc;
extern crate libxml;

// Bindings to the C interface
pub mod bindings;
/// Parser for loading an XSLT stylesheet from a file or string source.
pub mod parser;
/// Stylesheet functionality for Document transformation.
pub mod stylesheet;

/// Register the full set of EXSLT extension functions (str:*, math:*,
/// set:*, date:*) into libxslt's global extension registry. Must be
/// called before any stylesheet-application that uses EXSLT functions
/// (LaTeXML's stylesheets, for example, rely on `str:tokenize`). Safe to
/// call more than once — internally guarded by `std::sync::Once`, and
/// libexslt itself tolerates re-registration.
pub fn register_exslt() {
  use std::sync::Once;
  static REGISTER: Once = Once::new();
  REGISTER.call_once(|| unsafe {
    bindings::exsltRegisterAll();
  });
}
