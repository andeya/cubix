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
cubix = "0.3"
```

## Features

|       mod        |                                                     description                                                     |   source crate   |
| :--------------: | :-----------------------------------------------------------------------------------------------------------------: | :--------------: |
|  `api_response`  |                   A consistent structure for API responses, including success and error handling.                   |  `api-response`  |
| `convert_traits` |  Define your own conversion traits to solve the problem of converting two external types without using new types.   | `convert_traits` |
|   `jwt-claims`   | Structured version of the JWT Claims Set, as referenced at https://datatracker.ietf.org/doc/html/rfc7519#section-4. |   `jwt_claims`   |
