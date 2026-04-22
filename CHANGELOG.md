# Change Log

## [0.1.4] (in development)


## [0.1.3] 2026-22-04

* Link against `libexslt` (via pkg-config, with a `-lexslt` fallback) and auto-register the EXSLT extension functions (`str:*`, `math:*`, `set:*`, `date:*`) on the first `parser::parse_file` / `parser::parse_bytes` call, matching `xsltproc`'s default behaviour.
* New public `libxslt::register_exslt()` — an idempotent, thread-safe manual hook for callers that want deterministic early init. Internally guarded by `std::sync::Once`.

## [0.1.2] 2021-26-11

* Added `Parser::parse_bytes` and the ability to give parameters to `stylesheet::transform`, thanks @antmelnyk!
