// Copyright 2016 `multipart` Crate Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Client- and server-side abstractions for HTTP `multipart/form-data` requests.
//!
//! This is `multipart_ng`, a maintained fork of [`multipart`](https://crates.io/crates/multipart)
//! 0.18.0 with an identical API. Add it under the name `multipart` so that existing
//! `use multipart::...` imports keep working, and enable only the features you need:
//!
//! ```toml
//! [dependencies]
//! multipart = { package = "multipart_ng", version = "0.18", default-features = false, features = ["server"] }
//! ```
//!
//! The examples in this documentation use that name. Without the rename, write
//! `multipart_ng::` instead. The default features enable integrations for old framework
//! versions with known vulnerabilities in their dependency trees; see the
//! [README](https://github.com/CarloTUM/multipart_ng#installation) for details.
//!
//! ### Features:
//! This documentation is built with all features enabled.
//!
//! * `client`: The client-side abstractions for generating multipart requests.
//!
//! * `server`: The server-side abstractions for parsing multipart requests.
//!
//! * `mock`: Provides mock implementations of core `client` and `server` traits for debugging
//! or non-standard use.
//!
//! * `hyper`: Integration with the [Hyper](https://crates.io/crates/hyper) HTTP library
//! (0.9 and 0.10 only, the old synchronous API) for client and/or server depending on which
//! other feature flags are set.
//!
//! * `iron`: Integration with the [Iron](http://crates.io/crates/iron) web application
//! framework. See the [`server::iron`](server/iron/index.html) module for more information.
//!
//! * `nickel`: Integration with the [Nickel](https://crates.io/crates/nickel)
//! web application framework. See the [`server::nickel`](server/nickel/index.html) module for more
//! information.
//!
//! * `tiny_http`: Integration with the [`tiny_http`](https://crates.io/crates/tiny_http)
//! crate (0.6). See the [`server::tiny_http`](server/tiny_http/index.html) module for more information.
//!
//! ### Using it with other servers
//! The server side only needs a body implementing `std::io::Read` and the boundary from the
//! `Content-Type` header: use [`server::Multipart::with_body()`](server/struct.Multipart.html#method.with_body),
//! or implement [`server::HttpRequest`](server/trait.HttpRequest.html) for your request type.
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(clippy))]
#![cfg_attr(feature = "bench", feature(test))]
#![deny(missing_docs)]

#[macro_use]
extern crate log;

extern crate mime;
extern crate mime_guess;
extern crate rand;
extern crate tempfile;

#[cfg(feature = "quick-error")]
#[macro_use]
extern crate quick_error;

#[cfg(feature = "hyper")]
extern crate hyper;

#[cfg(feature = "iron")]
extern crate iron;

#[cfg(feature = "tiny_http")]
extern crate tiny_http;

#[cfg(test)]
extern crate env_logger;

#[cfg(any(feature = "mock", test))]
pub mod mock;

use rand::Rng;

/// Chain a series of results together, with or without previous results.
///
/// ```
/// #[macro_use] extern crate multipart_ng as multipart;
///
/// fn try_add_one(val: u32) -> Result<u32, u32> {
///     if val < 5 {
///         Ok(val + 1)
///     } else {
///         Err(val)
///     }
/// }
///
/// fn main() {
///     let res = chain_result! {
///         try_add_one(1),
///         prev -> try_add_one(prev),
///         prev -> try_add_one(prev),
///         prev -> try_add_one(prev)
///     };
///
///     println!("{:?}", res);
/// }
///
/// ```
#[macro_export]
macro_rules! chain_result {
    ($first_expr:expr, $($try_expr:expr),*) => (
        $first_expr $(.and_then(|_| $try_expr))*
    );
    ($first_expr:expr, $($($arg:ident),+ -> $try_expr:expr),*) => (
        $first_expr $(.and_then(|$($arg),+| $try_expr))*
    );
}

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;

#[cfg(all(test, feature = "client", feature = "server"))]
mod local_test;

fn random_alphanumeric(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(|c| c as char)
        .collect()
}

#[cfg(test)]
fn init_log() {
    let _ = env_logger::try_init();
}
