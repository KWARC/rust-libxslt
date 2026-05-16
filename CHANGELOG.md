# Change Log

## [0.1.4] 2026-05-16

* **Docs: explicitly call out `Stylesheet` as `!Send + !Sync`.** No API change — the wrapper has always been single-threaded by default (`xsltStylesheetPtr` is a raw `*mut`, which makes the auto-traits negative) — but the new doc block makes the contract explicit and walks callers through the two supported reuse patterns: park the parsed stylesheet in `thread_local!` storage for per-thread reuse, or wrap in a `Mutex` for cross-thread sharing.
* An earlier draft of this release relaxed `Stylesheet::transform` from `&mut self` to `&self` and added `unsafe impl Send + Sync`. We rolled that back: libxslt is not documented as thread-safe, and the apply path is not audited to be read-only on the stylesheet (it may write back into namespace-internalisation caches or error context fields). The same caution that resolved issue #6 (libxslt's hidden mutation of the input `Document`) applies to the stylesheet object too. The `&mut self` requirement remains, sequential reuse from a single thread is the supported pattern.

## [0.1.3] 2026-22-04

* Link against `libexslt` (via pkg-config, with a `-lexslt` fallback) and auto-register the EXSLT extension functions (`str:*`, `math:*`, `set:*`, `date:*`) on the first `parser::parse_file` / `parser::parse_bytes` call, matching `xsltproc`'s default behaviour.
* New public `libxslt::register_exslt()` — an idempotent, thread-safe manual hook for callers that want deterministic early init. Internally guarded by `std::sync::Once`.
* **Breaking, soundness fix (#6)**: `Stylesheet::transform` now takes the input `Document` *by value* (`doc: Document`) instead of by shared reference (`doc: &Document`). libxslt can mutate the input while applying stylesheet-controlled whitespace stripping; exposing that mutation through `&Document` was undefined behaviour reachable from safe code. Call sites should pass `source` where they previously passed `&source`; clone the `Document` up front if you need to transform it through multiple stylesheets.

## [0.1.2] 2021-26-11

* Added `Parser::parse_bytes` and the ability to give parameters to `stylesheet::transform`, thanks @antmelnyk!
