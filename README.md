# Sortable (timestamp-first) unique identifier

[![Tests](https://img.shields.io/github/actions/workflow/status/abineo/suid/rust.yml?branch=main)](https://github.com/abineo/suid)
[![Crates.io](https://img.shields.io/crates/v/suid)](https://crates.io/crates/suid)
[![Documentation](https://docs.rs/suid/badge.svg)](https://docs.rs/suid)

## Why?

Random identifiers can be bad for performance when inserting into [_balanced_ b-tree](https://en.wikipedia.org/wiki/B-tree) indexes.

## Variants

| variant        | timestamp bits                           | random bits |
| -------------- | ---------------------------------------- | ----------- |
| `u32`, `i32`   | **16** _(hours since unix epoch)_        | **16**      |
| `u64`, `i64`   | **40** _(seconds since unix epoch)_      | **24**      |
| `u128`, `i128` | **64** _(milliseconds since unix epoch)_ | **64**      |

## License

[☕ Coffee License 2.0](https://coffee-license.org/v2.0)
