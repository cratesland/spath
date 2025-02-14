# SPath: Query expressions for semi-structured data

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MSRV 1.80][msrv-badge]](https://www.whatrustisit.com)
[![Apache 2.0 licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/spath.svg
[crates-url]: https://crates.io/crates/spath
[docs-badge]: https://docs.rs/spath/badge.svg
[msrv-badge]: https://img.shields.io/badge/MSRV-1.80-green?logo=rust
[docs-url]: https://docs.rs/spath
[license-badge]: https://img.shields.io/crates/l/spath
[license-url]: LICENSE
[actions-badge]: https://github.com/cratesland/spath/workflows/CI/badge.svg
[actions-url]:https://github.com/cratesland/spath/actions?query=workflow%3ACI

## Overview

You can use it as a drop-in replacement for JSONPath, but also for other semi-structured data formats like TOML or user-defined variants.

## Documentation

* [API documentation on docs.rs](https://docs.rs/spath)

## Example

Here is a quick example that shows how to use the `spath` crate to query JSONPath alike expression over JSON data:

```rust
use serde_json::json;
use spath::SPath;

#[test]
fn main() {
    let data = json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ]
    });

    let registry = spath::json::BuiltinFunctionRegistry::default();
    let spath = SPath::parse_with_registry("$.phones[1]", registry).unwrap();
    let result = spath.query(&data);
    let result = result.exactly_one().unwrap();
    assert_eq!(result, &json!("+44 2345678"));
}
```

## Usage

`spath` is [on crates.io](https://crates.io/crates/spath) and can be used by adding `spath` to your dependencies in your project's `Cargo.toml`. Or more simply, just run `cargo add spath`.

## License

This project is licensed under [Apache License, Version 2.0](LICENSE).

## History

From 0.3.0, this crate is reimplemented as a fork of [serde_json_path](https://crates.io/crates/serde_json_path), with modifications:

* Support other semi-structured data values
* Rewrite the parser with winnow + logos
* Redesign the function registry
* `impl Ord for PathElement`
* Drop Integer wrapper (although it's a MUST in RFC 9535, I don't find the reason and highly suspect it's because JSON has only numbers (IEEE 754 float))
* Drop serde related impls.
