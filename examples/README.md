`multipart_ng` Examples
=======================

These example files show how to use `multipart_ng` with the various crates it integrates with.
They import the crate as `multipart` (`extern crate multipart_ng as multipart;`), which is what
you get with the `package = "multipart_ng"` rename recommended in the [main README](../README.md).

The integrations target old framework versions (hyper 0.9/0.10, iron 0.4–0.6, tiny_http 0.6,
Rocket 0.4). The Rocket example needs a nightly compiler, as Rocket 0.4 does.

These files carry the same licenses as [`multipart_ng` itself](../README.md#license).

## Client

Examples for the client-side integrations of `multipart`'s API.

[`hyper_client`](hyper_client.rs)
---------------------------------
Author: [abonander]

This example showcases usage of `multipart` with the `hyper::client::Request` API.

```
$ cargo run --example hyper_client
```

[`hyper_reqbuilder`](hyper_reqbuilder.rs)
-----------------------------------------
Author: [abonander]

This example showcases usage of `multipart` with Hyper's new `Client` API,
via the lazy-writing capabilities of `multipart::client::lazy`.

```
$ cargo run --example hyper_reqbuilder
```

## Server

[`hyper_server`](hyper_server.rs)
---------------------------------
Author: [Puhrez]

This example shows how to use `multipart` with a [`hyper::Server`] (http://hyper.rs/) to intercept multipart requests.

```
$ cargo run --example hyper_server
```

[`iron`](iron.rs)
-----------------
Author: [White-Oak]

This example shows how to use `multipart` with the [Iron web application framework](https://github.com/iron/iron), via `multipart`'s support
for the `iron::Request` type.

To run:

```
$ cargo run --features iron --example iron
```

[`iron_intercept`](iron_intercept.rs)
-------------------------------------
Author: [abonander]

This example shows how to use `multipart`'s specialized `Intercept` middleware with Iron, which reads out all fields and
files to local storage so they can be accessed arbitrarily.

```
$ cargo run --features iron --example iron_intercept
```

[`tiny_http`](tiny_http.rs)
---------------------------
Author: [White-Oak]

This example shows how to use `multipart` with the [`tiny_http` crate](https://crates.io/crates/tiny_http), via `multipart`'s support for the `tiny_http::Request` type.

```
$ cargo run --features tiny_http --example tiny_http
```

[`hyper_server`](hyper_server.rs)
---------------------------------
Author: [Puhrez]

This example shows how to use `multipart` with a [`hyper::Server`] (http://hyper.rs/) to intercept multipart requests.

```
$ cargo run --example hyper_server
```

[`nickel`](nickel.rs)
---------------------
Author: [iamsebastian]

This example shows how to use `multipart` to handle multipart uploads in [nickel.rs](https://nickel.rs).

```
$ cargo run --example nickel --features nickel
```

[Rocket](rocket.rs)
-------------------
Author: [abonander]

This example shows how `multipart`'s server API can be used with [Rocket](https://rocket.rs) 0.4
without explicit support. (Rocket 0.5 and later handle `multipart/form-data` themselves.)

```
$ cargo run --example rocket --features "rocket"
```

[iamsebastian]: https://github.com/iamsebastian
[Puhrez]: https://github.com/puhrez
[White-Oak]: https://github.com/white-oak
[abonander]: https://github.com/abonander

