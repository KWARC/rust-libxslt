//! Base API tests, to be split into distinct sub-suites later on
//!
extern crate libxml;
extern crate libxslt;

use libxml::parser::Parser as XMLParser;
use libxml::tree::Document;
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

  let source_len = source.to_string(false).len();
  assert!(source_len > 1000);

  let new_doc_result = stylesheet.transform(&source);
  assert!(new_doc_result.is_ok());
  let new_doc: Document = new_doc_result.unwrap();
  let new_len = new_doc.to_string(true).len();
  assert!(new_len > 1600);
  // This particular example converts an XML document into an HTML one with a custom header, which happens to be ~600 chars longer
  assert!(new_len > source_len);
}
