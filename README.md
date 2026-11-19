# multipart_ng

[![crates.io](https://img.shields.io/crates/v/multipart_ng.svg)](https://crates.io/crates/multipart_ng)
[![docs.rs](https://docs.rs/multipart_ng/badge.svg)](https://docs.rs/multipart_ng)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/multipart_ng.svg)](#license)
[![CI](https://github.com/CarloTUM/multipart_ng/actions/workflows/ci.yml/badge.svg)](https://github.com/CarloTUM/multipart_ng/actions/workflows/ci.yml)

Client- and server-side `multipart/form-data` handling (HTTP file uploads) with a synchronous,
`std::io`-based API.

`multipart_ng` is a maintained fork of [`multipart`](https://crates.io/crates/multipart) 0.18.0,
whose repository is archived
([RUSTSEC-2023-0050](https://rustsec.org/advisories/RUSTSEC-2023-0050.html)).
The API matches `multipart` 0.18.0 exactly. The only code change fixes the warning
*"the following packages contain code that will be rejected by a future version of Rust:
buf_redux v0.8.4, multipart v0.18.0"* (lint `trailing_semicolon_in_macro`), both here and in
the dependency, which is replaced by [`buf_redux_ng`](https://crates.io/crates/buf_redux_ng).

## Installation

```toml
[dependencies]
multipart = { package = "multipart_ng", version = "0.18", default-features = false, features = ["server"] }
```

Use `features = ["client"]` to build requests, `["server"]` to parse them, or both.
The `package = "multipart_ng"` rename keeps your existing `use multipart::...` imports working.
Without the rename, import from `multipart_ng::` instead.

> **Turn off the default features.** They enable the integrations for hyper 0.10, iron, nickel
> and tiny_http 0.6. Those pull in about 130 crates, and as of November 2026 `cargo audit` reports
> 4 vulnerabilities for them, including two HTTP parsing bugs in hyper 0.10
> ([RUSTSEC-2021-0078](https://rustsec.org/advisories/RUSTSEC-2021-0078.html), request smuggling;
> [RUSTSEC-2021-0079](https://rustsec.org/advisories/RUSTSEC-2021-0079.html)).
> With `default-features = false, features = ["server"]`, the dependency tree is about
> 35 crates and `cargo audit` reports no vulnerabilities, only informational warnings that
> `safemem` and `twoway` are no longer maintained. With `features = ["client"]` it reports
> nothing at all. The defaults also pull in `typemap` (through iron and nickel), which triggers
> its own future-incompatibility warning.
> The defaults stay unchanged for compatibility with `multipart` 0.18.0.

## Migrating from `multipart`

Replace the dependency line:

```diff
- multipart = { version = "0.18", default-features = false, features = ["server"] }
+ multipart = { package = "multipart_ng", version = "0.18", default-features = false, features = ["server"] }
```

You don't need to change any code.

**Getting `multipart` through another crate**, e.g. [`rouille`](https://crates.io/crates/rouille)?
A `[patch]` section can't do this swap, because Cargo only patches a package with another
package of the same name. The crate that depends on `multipart` has to switch. Its maintainers
only need to change the one line shown above, so it's worth opening an issue there.

## Example

This example builds a request body on the client side and parses it again on the server side,
without any HTTP library. Enable both `client` and `server` to run it:

```rust
use multipart::client::lazy::Multipart as Form;
use multipart::server::Multipart;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Client side: build a `multipart/form-data` body.
    let mut form = Form::new();
    form.add_text("greeting", "hello")
        .add_stream("upload", &b"file contents"[..], Some("hello.txt"), None);
    let body = form.prepare().map_err(|e| e.error)?;
    let boundary = body.boundary().to_owned();

    // Server side: parse it again. In a real server, `body` is the request body
    // and `boundary` comes from the `Content-Type` header.
    let mut multipart = Multipart::with_body(body, boundary);
    multipart.foreach_entry(|mut field| {
        let mut data = String::new();
        field.data.read_to_string(&mut data).unwrap();
        println!("{} ({:?}): {}", field.headers.name, field.headers.filename, data);
    })
}
```

Header values such as `field.headers.filename` come from the client and are not validated.
Never use them as file system paths without sanitizing them. To write uploads to disk with
a per-field size limit, use `multipart.save().size_limit(...)` (see the
[`server::save`](https://docs.rs/multipart_ng/latest/multipart_ng/server/save/index.html) docs).

## Using it with any HTTP server

The server side needs only two things: a body that implements `std::io::Read` and the boundary
from the `Content-Type: multipart/form-data; boundary=...` header.

- Call `server::Multipart::with_body(body, boundary)` directly, as in the example above, or
- implement `server::HttpRequest` for your request type and call
  `server::Multipart::from_request(req)`.

This works with any framework, sync or async, as long as you can hand the body over as a
blocking `Read`.

## Feature flags

| Feature     | Default | Description |
|-------------|---------|-------------|
| `client`    | yes     | Build `multipart/form-data` requests (`client::lazy::Multipart`, `client::Multipart`). |
| `server`    | yes     | Parse `multipart/form-data` requests (`server::Multipart`) and save fields to disk (`server::save`). |
| `mock`      | yes     | Mock client and server types for testing and debugging. |
| `hyper`     | yes     | Client and server integration for **hyper 0.9 / 0.10** (old synchronous API only). |
| `iron`      | yes     | Server integration for **iron 0.4 – 0.6**, including the `server::iron::Intercept` middleware. |
| `nickel`    | yes     | Server integration for **nickel**. |
| `tiny_http` | yes     | Server integration for **tiny_http 0.6**. |

The framework integrations target the framework versions listed. For current versions of these
frameworks, or any other server, use the generic API described in
[Using it with any HTTP server](#using-it-with-any-http-server).

The [`examples/`](examples) directory has one example per integration, plus one showing how to use
the crate with Rocket 0.4. Run them with `cargo run --example <name>`.

## Scope of this fork

- The public API of `multipart` 0.18.0 stays as it is. The 0.18.x line gets no new features and
  no breaking changes.
- Compatibility fixes for new Rust releases and security fixes are in scope. Please report
  problems on the [issue tracker](https://github.com/CarloTUM/multipart_ng/issues).
- Cargo produces no future-incompatibility report for this crate on current stable Rust
  (checked with 1.99).

## Built on

- [`buf_redux_ng`](https://crates.io/crates/buf_redux_ng): `BufReader` with a minimum-buffered
  policy. A field boundary can straddle two reads, so the buffer must be refilled before it runs
  empty.
- [`httparse`](https://crates.io/crates/httparse): zero-copy parsing of field headers.
- [`twoway`](https://crates.io/crates/twoway): fast substring search for finding boundaries.

## Credits

`multipart` was written by Austin Bonander and contributors
([abonander/multipart](https://github.com/abonander/multipart), archived).
All credit for the design and implementation goes to them.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
