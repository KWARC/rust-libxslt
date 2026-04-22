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
/// set:*, date:*) into libxslt's global extension registry.
///
/// You do not normally need to call this: [`parser::parse_file`] and
/// [`parser::parse_bytes`] invoke it on the first stylesheet load, so
/// EXSLT is available to every stylesheet this crate parses. The
/// function is exposed for callers who want deterministic early init
/// (tests, embedding frameworks, or code paths that construct
/// stylesheets through other bindings).
///
/// Safe to call more than once and from multiple threads: concurrent
/// callers are serialized by `std::sync::Once`, so `exsltRegisterAll`
/// runs exactly once per process. libexslt additionally tolerates
/// re-registration as a belt-and-braces measure.
pub fn register_exslt() {
  use std::sync::Once;
  static REGISTER: Once = Once::new();
  REGISTER.call_once(|| unsafe {
    bindings::exsltRegisterAll();
  });
}
