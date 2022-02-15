use std::ffi::CString;
use std::path::Path;

use crate::bindings::{xsltParseStylesheetDoc, xsltParseStylesheetFile};
use crate::libxml::bindings::xmlReadMemory;

use crate::stylesheet::Stylesheet;

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

/// Load an XSLT stylesheet from UTF-8 string in byte format
pub fn parse_bytes(file_string_as_bytes: Vec<u8>, url: &str) -> Result<Stylesheet, String> {
    unsafe {
        let xsl_file_string_len = file_string_as_bytes.len() as i32;
        let xsl_file_c_str = CString::new(file_string_as_bytes).unwrap();
        let url_c_str = CString::new(url).unwrap();

        let bytes = xsl_file_c_str.as_bytes_with_nul();
        let ptr = bytes.as_ptr();
        let file = ptr as *const libc::c_char;

        let bytes = url_c_str.as_bytes_with_nul();
        let ptr = bytes.as_ptr();
        let url = ptr as *const libc::c_char;

        let xml = xmlReadMemory(file, xsl_file_string_len, url, std::ptr::null::<libc::c_char>(), 0);

        let ptr = xsltParseStylesheetDoc(xml);

        Ok(Stylesheet { ptr })
    }
}
