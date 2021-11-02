//! Base API tests, to be split into distinct sub-suites later on
//!
extern crate libxml;
extern crate libxslt;

use std::fs;

use libxml::tree::Document;
use libxml::{parser::Parser as XMLParser, tree::SaveOptions};
use libxslt::parser as xslt_parser;

// PROOF OF CONCEPT

#[test]
/// Build a hello world XML doc
fn hello_builder() {
    let xml_parser = XMLParser::default();

    let source_result = xml_parser.parse_file("tests/data/1.xml");
    assert!(source_result.is_ok());
    let source = source_result.unwrap();

    // Parse XSL file from the file system
    let stylesheet_result = xslt_parser::parse_file("tests/data/1.xsl");
    assert!(stylesheet_result.is_ok());
    let mut stylesheet = stylesheet_result.unwrap();

    // Parse XSL file from memory
    let stylesheet_string_bytes = fs::read("tests/data/1.xsl").unwrap();
    let stylesheet_from_string_result = xslt_parser::parse_string(stylesheet_string_bytes, "1");
    assert!(stylesheet_from_string_result.is_ok());
    let mut stylesheet_from_string = stylesheet_from_string_result.unwrap();

    let source_len = source.to_string().len();
    assert!(source_len > 1000);

    let new_doc_result = stylesheet.transform(&source, vec![("yearfrom", "1234")]);
    let new_doc_from_string_result = stylesheet_from_string.transform(&source, vec![("yearfrom", "1234")]);
    assert!(new_doc_result.is_ok());
    assert!(new_doc_from_string_result.is_ok());
    let new_doc: Document = new_doc_result.unwrap();
    let new_doc_from_string: Document = new_doc_from_string_result.unwrap();

    assert_eq!(new_doc.to_string(), new_doc_from_string.to_string());

    let new_serialized = new_doc.to_string_with_options(SaveOptions {
        format: true,

        ..SaveOptions::default()
    });
    let new_len = new_serialized.len();

    std::fs::write("result.xsl", new_serialized).unwrap();

    assert!(new_len > 1500);
    // This particular example converts an XML document into an HTML one with a custom header, which happens to be ~600 chars longer
    assert!(new_len > source_len);
}
