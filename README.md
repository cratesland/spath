# SPath: Query expressions for semi-structured data

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MSRV 1.75][msrv-badge]](https://www.whatrustisit.com)
[![Apache 2.0 licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/spath.svg
[crates-url]: https://crates.io/crates/spath
[docs-badge]: https://docs.rs/spath/badge.svg
[msrv-badge]: https://img.shields.io/badge/MSRV-1.75-green?logo=rust
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
use serde_json::Value as JsonValue;
use spath::SPath;
use spath::Value;

fn main() {
    let data = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let spath = SPath::new("$.phones[1]").unwrap();
    let value = Value::from(data);
    let result = spath.eval(&value).unwrap();
    assert_eq!(JsonValue::from(result), json!("+44 2345678"));
}
```

## Usage

`spath` is [on crates.io](https://crates.io/crates/spath) and can be used by adding `spath` to your dependencies in your project's `Cargo.toml`. Or more simply, just run `cargo add spath`.

## License

This project is licensed under [Apache License, Version 2.0](LICENSE).
