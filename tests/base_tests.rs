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
    let stylesheet_result = xslt_parser::parse_file("tests/data/1.xsl");
    assert!(stylesheet_result.is_ok());
    let mut stylesheet = stylesheet_result.unwrap();

    let source_len = source.to_string().len();
    assert!(source_len > 1000);

    let new_doc_result = stylesheet.transform(&source, Vec::new());
    assert!(new_doc_result.is_ok());
    let new_doc: Document = new_doc_result.unwrap();
    let new_serialized = new_doc.to_string_with_options(SaveOptions {
        format: true,

        ..SaveOptions::default()
    });
    let new_len = new_serialized.len();
    assert!(new_len > 1500);
    // This particular example converts an XML document into an HTML one with a custom header, which happens to be ~600 chars longer
    assert!(new_len > source_len);
}

#[test]
fn from_string_bytes_builder() {
    let xml_parser = XMLParser::default();

    let source_result = xml_parser.parse_file("tests/data/1.xml");
    assert!(source_result.is_ok());
    let source = source_result.unwrap();

    let source_bytes = fs::read("tests/data/1.xsl").unwrap();
    let stylesheet_result = xslt_parser::parse_bytes(source_bytes, "test");
    assert!(stylesheet_result.is_ok());
    let mut stylesheet = stylesheet_result.unwrap();

    let xslt_params = vec![
        ("yearfrom", "1999"),
        ("yearto", "2000")
    ];

    let new_doc_result = stylesheet.transform(&source, xslt_params);
    assert!(new_doc_result.is_ok());
    let new_doc: Document = new_doc_result.unwrap();
    let new_serialized = new_doc.to_string_with_options(SaveOptions {
        format: true,
        ..SaveOptions::default()
    });

    let new_len = new_serialized.len();
    assert!(new_len > 1500);
}
