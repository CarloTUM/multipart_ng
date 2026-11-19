# Changelog

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
