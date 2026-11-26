# Changelog

## 0.18.2 (2026-11-26)

Bug fixes and dependency cleanup. No API changes.

- Server: the CRLF in front of a boundary no longer ends up in the field data when it sits at
  the end of the read buffer
  ([abonander/multipart#139](https://github.com/abonander/multipart/issues/139)).
- Client: `PreparedFields::boundary()` no longer panics on an empty form, for example after a
  second call to `Multipart::prepare()`
  ([abonander/multipart#122](https://github.com/abonander/multipart/issues/122)).
- Client: `PreparedFields::content_len()` now includes the text fields. Before, it was too
  small for every form that had any.
- Mock: `StdoutTee` no longer fails a write when stdout rejects non-UTF-8 data, as the Windows
  console does.
- Replaced the unmaintained `twoway` and `safemem` dependencies with `memchr`, and require
  `buf_redux_ng` 0.8.6, which dropped `safemem` as well. With
  `default-features = false, features = ["server"]`, `cargo audit` now reports nothing.
- Fixed typos in the examples.

## 0.18.1 (2026-11-19)

No library code changes.

- Rewrote the README: installation with recommended features, migration guide, generic server
  usage, feature flags, notes on the advisories pulled in by the default features.
- Crate docs explain the fork and the `package = "multipart_ng"` rename; removed outdated notes.
- Examples, binaries and doctests now compile again (they still referred to the crate as
  `multipart`).
- Removed the `path` dependency on a local `buf_redux_ng` checkout; the repository now builds
  on its own.
- Updated crates.io description, keywords and categories.
- Replaced Travis CI with GitHub Actions (Linux, macOS, Windows; weekly run against the
  latest stable Rust).

## 0.18.0 (2026-05-31)

Fork of `multipart` 0.18.0.

- Removed the trailing semicolon in the `invalid_cont_disp!` macro, which triggered the
  `trailing_semicolon_in_macro` future-incompatibility warning.
- Depends on `buf_redux_ng` instead of `buf_redux` (same fix).
