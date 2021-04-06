# Lakh Crore Numbers

[![Build Status](https://github.com/suvash/lakh-crore-numbers/actions/workflows/.github/workflows/run-python-tests.yml/badge.svg)](https://github.com/suvash/lakh-crore-numbers/actions/workflows/run-python-tests.yml)
[![Build Status](https://github.com/suvash/lakh-crore-numbers/actions/workflows/.github/workflows/run-rust-tests.yml/badge.svg)](https://github.com/suvash/lakh-crore-numbers/actions/workflows/run-rust-tests.yml)


A collection of libraries in various programming languages for formatting numbers into the "Lakh Crore" (...हजार, लाख, करोड, अर्ब...) system.

This numbering system (also known as Indian numbering system) is widely used in the subcontinent, including Bangladesh, Bhutan, India, Maldives, Nepal, Pakistan and Sri Lanka [to express large numbers](https://en.wikipedia.org/wiki/Indian_numbering_system). The terms `lakh` and `crore` are commonly used in everyday scenarios. The library currently supports formatting all the way up to `sankha`.


## Supported Language(s)

As of now, the library is written for the following languages. Support for more subcontinental languages across other programming languages will be added over time. Please feel free to open a PR for a language you want this library to support.

### Programming Languages

- [Python (3.6 and up)](python)
- [Rust (edition 2018)](rust)

### Spoken Languages

- Nepali / नेपाली

## Documentation

The current goal of the library is to be format numbers into two types of representation.
- Number expressed as numeral in the target language
- Number expressed as words in the target language

A handful of functions are made available to facilitate this. Please take a look at the individual language documentation (as mentioned above) for more details.


## License

Copyright © 2021, Suvash Thapaliya

Distributed under the [ISC License](https://github.com/suvash/lakh-crore-numbers/blob/main/LICENSE).
