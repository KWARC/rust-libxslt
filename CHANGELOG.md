# Change Log

## [0.1.4] 2026-06-11

* Opt-in **static linking** of libxslt + libexslt. Set the `LIBXSLT_STATIC`
  environment variable and `build.rs` probes via pkg-config with
  `.statik(true)` (libexslt first, so its archive precedes the xslt/xml2 it
  depends on), emitting `static=` link directives. Point `PKG_CONFIG_PATH`
  at a non-system prefix holding PIC `libxslt.a` + `libexslt.a` to get a
  binary with no runtime `libxslt.so` dependency. Unset (the default) is
  unchanged: the original dynamic link.

## [0.1.3] 2026-22-04

* Link against `libexslt` (via pkg-config, with a `-lexslt` fallback) and auto-register the EXSLT extension functions (`str:*`, `math:*`, `set:*`, `date:*`) on the first `parser::parse_file` / `parser::parse_bytes` call, matching `xsltproc`'s default behaviour.
* New public `libxslt::register_exslt()` — an idempotent, thread-safe manual hook for callers that want deterministic early init. Internally guarded by `std::sync::Once`.
* **Breaking, soundness fix (#6)**: `Stylesheet::transform` now takes the input `Document` *by value* (`doc: Document`) instead of by shared reference (`doc: &Document`). libxslt can mutate the input while applying stylesheet-controlled whitespace stripping; exposing that mutation through `&Document` was undefined behaviour reachable from safe code. Call sites should pass `source` where they previously passed `&source`; clone the `Document` up front if you need to transform it through multiple stylesheets.

## [0.1.2] 2021-26-11

* Added `Parser::parse_bytes` and the ability to give parameters to `stylesheet::transform`, thanks @antmelnyk!
