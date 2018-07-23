use std::ffi::CString;
use std::path::Path;

use bindings::*;
use stylesheet::Stylesheet;

/// Load an XSLT stylesheet from (typically `.xsl`) file.
pub fn parse_file(path_str: &str) -> Result<Stylesheet, String> {
  let path = Path::new(path_str);
  if !path.is_file() {
    Err(format!(
      "Path {:?} does not point to a valid file on the file system.",
      path_str
    ))
  } else {
    unsafe {
      let c_path_str = CString::new(path_str).unwrap();
      let ptr = xsltParseStylesheetFile(c_path_str.as_bytes().as_ptr());
      if ptr.is_null() {
        Err(format!("Failed to parse stylesheet file {:?}", path_str))
      } else {
        Ok(Stylesheet { ptr })
      }
    }
  }
}
