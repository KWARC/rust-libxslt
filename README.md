[![CI](https://github.com/KWARC/rust-libxslt/actions/workflows/ci.yml/badge.svg)](https://github.com/KWARC/rust-libxslt/actions/workflows/ci.yml)
[![API Documentation](https://img.shields.io/badge/docs-API-blue.svg)](http://KWARC.github.io/rust-libxslt/libxslt/index.html)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/KWARC/rust-libxslt/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/libxslt.svg)](https://crates.io/crates/libxslt)

Rust wrapper for [libxslt](http://xmlsoft.org/), and a sibling crate to [libxml](https://crates.io/crates/libxml).

The main goal of this project is to benefit from libxslt's maturity and stability while native Rust XSLT crates mature to be near-drop-in replacements.

## Installation

The crate links against `libxslt` and `libexslt` (for the EXSLT extension functions) via `pkg-config`. On modern Debian/Ubuntu, Fedora, and macOS Homebrew, libexslt ships inside the main libxslt development package — a single install is enough:

* Debian / Ubuntu: `apt install libxml2-dev libxslt1-dev`
* Fedora / RHEL: `dnf install libxml2-devel libxslt-devel`
* macOS (Homebrew): `brew install libxml2 libxslt`

**Coverage**: This is an infant proof of concept in both coverage and feature richness, **NOT** ready for production use.

**Welcome!** With these caveats, the contributors to the project are migrating production work towards Rust and find a continuing reliance on libxslt a helpful relief for initial ports. As such, contributions to this crate are welcome, if your workflow is not yet fully supported.
