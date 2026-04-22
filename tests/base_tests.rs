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

    let new_doc_result = stylesheet.transform(source, Vec::new());
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

    let new_doc_result = stylesheet.transform(source, xslt_params);
    assert!(new_doc_result.is_ok());
    let new_doc: Document = new_doc_result.unwrap();
    let new_serialized = new_doc.to_string_with_options(SaveOptions {
        format: true,
        ..SaveOptions::default()
    });

    let new_len = new_serialized.len();
    assert!(new_len > 1500);
}

#[test]
/// Apply a stylesheet that uses `str:tokenize` (EXSLT) to verify that
/// parser-triggered auto-registration of libexslt works end-to-end.
/// Deliberately does *not* call `register_exslt()` manually — if it
/// did, the test would pass even if auto-registration regressed.
fn exslt_str_tokenize_auto_registers() {
    const XSL: &[u8] = br#"<?xml version="1.0"?>
<xsl:stylesheet version="1.0"
    xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
    xmlns:str="http://exslt.org/strings"
    extension-element-prefixes="str">
  <xsl:output method="xml" indent="no"/>
  <xsl:template match="/">
    <tokens>
      <xsl:for-each select="str:tokenize('a,b,c,d', ',')">
        <token><xsl:value-of select="."/></token>
      </xsl:for-each>
    </tokens>
  </xsl:template>
</xsl:stylesheet>"#;

    let source = XMLParser::default()
        .parse_string("<root/>")
        .expect("parse trivial source xml");
    let mut stylesheet = xslt_parser::parse_bytes(XSL.to_vec(), "exslt_tokenize.xsl")
        .expect("parse exslt stylesheet");

    let output = stylesheet
        .transform(source, Vec::new())
        .expect("transform with str:tokenize")
        .to_string();

    for tok in ["<token>a</token>", "<token>b</token>", "<token>c</token>", "<token>d</token>"] {
        assert!(
            output.contains(tok),
            "expected {tok} in EXSLT output, got: {output}"
        );
    }
}
