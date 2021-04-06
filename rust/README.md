# Lakh Crore Numbers - Rust

[![Build Status](https://github.com/suvash/lakh-crore-numbers/actions/workflows/.github/workflows/run-rust-tests.yml/badge.svg)](https://github.com/suvash/lakh-crore-numbers/actions/workflows/run-rust-tests.yml)

A Rust crate for formatting numbers into the "Lakh Crore" (...हजार, लाख, करोड, अर्ब...) system.

## Installation

lakh-crore-numbers is available on crates.io:

```toml
lakh-crore-numbers = "0.1.0"
```

## Documentation

## Formatting to Numerals (Nepali / नेपाली)

```rust
use lakh_crore_numbers as lcn;

assert_eq!(lcn::format_to_nepali_numeral(295678), "२९५६७८");
```

## Formatting to Words (Nepali / नेपाली)

```rust
use lakh_crore_numbers as lcn;

assert_eq!(
  lcn::format_to_nepali_words(8359),
  Ok(String::from("आठ हजार तीन सय उनन्साट्ठी")));
```

Please take a look at the [official documentation](https://docs.rs/lakh-crore-numbers/0.1.0/lakh_crore_numbers/) as well.


## Changelog

Please check the [CHANGELOG.md](https://github.com/suvash/lakh-crore-numbers/blob/main/rust/CHANGELOG.md) for details.


## License

Copyright © 2021, Suvash Thapaliya

Distributed under the [ISC License](https://github.com/suvash/lakh-crore-numbers/blob/main/LICENSE).
