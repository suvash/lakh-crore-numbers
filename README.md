# Lakh Crore Numbers

A Python library for formatting numbers into the "Lakh Crore" (...हजार, लाख, करोड, अर्ब...) system.

This numbering system (also known as Indian numbering system) is widely used in the subcontinent, including Bangladesh, Bhutan, India, Maldives, Nepal, Pakistan and Sri Lanka [to express large numbers](https://en.wikipedia.org/wiki/Indian_numbering_system). The terms `lakh` and `crore` are commonly used in everyday scenarios. The library currently supports formatting all the way up to `sankha`.


## Installation

lakh-crore-numbers is available on PyPI:

```bash
 python -m pip install lakh-crore-numbers
```

## Supported Language(s)

As of now, the library is written for the following languages. Support for more subcontinental languages across other programming languages will be added over time. Please feel free to open a PR for a language you want this library to support.

### Programming Languages

- Python (3.6 and up)

### Spoken Languages

- Nepali / नेपाली

## Documentation

The current goal of the library is to be format numbers into two types of representation.
- Number expressed as numeral in the target language
- Number expressed as words in the target language

A handful of functions are available on the top level import to facilitate this.

## Formatting to Numerals (Nepali / नेपाली)

```python
import lakh_crore_numbers as lcn

assert lcn.format_to_nepali_numeral(783456) == '७८३४५६'
```

## Formatting to Words (Nepali / नेपाली)

```python
import lakh_crore_numbers as lcn

assert lcn.format_to_nepali_words(901489783456) == 'नौ खर्ब एक अर्ब अठचालीस करोड सन्तानब्बे लाख त्रियासी हजार चार सय छपन्न'
```

For more formatting examples, please [take a look here](https://github.com/suvash/lakh-crore-numbers/blob/main/python/tests/test_format.py).


## Changelog

Please check the [CHANGELOG.md](https://github.com/suvash/lakh-crore-numbers/blob/main/python/CHANGELOG.md) for details.


## License

Copyright © 2021, Suvash Thapaliya

Distributed under the [ISC License](https://github.com/suvash/lakh-crore-numbers/blob/main/LICENSE).
