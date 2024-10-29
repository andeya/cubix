# cubix

Standardized building block components born for Rust craft.

[![GitHub last commit](https://img.shields.io/github/last-commit/andeya/cubix)](https://github.com/andeya/cubix/commits/main)
[![Crates.io](https://img.shields.io/crates/v/cubix.svg)](https://crates.io/crates/cubix)
[![Docs](https://docs.rs/cubix/badge.svg)](https://docs.rs/cubix)

## Usage

Run the following Cargo command in your project directory:

```sh
cargo add cubix
```

Or add the following line to your Cargo.toml:

```toml
cubix = "0.6"
```

## Features

|       mod        |                                                     description                                                     |                      source crate                       |
| :--------------: | :-----------------------------------------------------------------------------------------------------------------: | :-----------------------------------------------------: |
|  `api_response`  |                   A consistent structure for API responses, including success and error handling.                   | [`api-response`](https://crates.io/crates/api-response) |
| `convert_traits` |  Define your own conversion traits to solve the problem of converting two external types without using new types.   |      [`convert_traits`](https://crates.io/crates/)      |
|    `getset2`     |                   A procedural macro for generating the most basic getters and setters on fields.                   |      [`getset2`](https://crates.io/crates/getset2)      |
|   `jwt-claims`   | Structured version of the JWT Claims Set, as referenced at https://datatracker.ietf.org/doc/html/rfc7519#section-4. |   [`jwt_claims`](https://crates.io/crates/jwt_claims)   |
|    `num_enum`    |   Procedural macros to make inter-operation between primitives and enums easier. This crate is no_std compatible.   |     [`num_enum`](https://crates.io/crates/num_enum)     |
|     `strum`      |               Strum is a set of macros and traits for working with enums and strings easier in Rust.                |        [`strum`](https://crates.io/crates/strum)        |
