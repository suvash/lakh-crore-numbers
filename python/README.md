# Lakh Crore Numbers - Python

[![Build Status](https://github.com/suvash/lakh-crore-numbers/actions/workflows/.github/workflows/run-python-tests.yml/badge.svg)](https://github.com/suvash/lakh-crore-numbers/actions/workflows/run-python-tests.yml)

A Python library for formatting numbers into the "Lakh Crore" (...हजार, लाख, करोड, अर्ब...) system.

## Installation

lakh-crore-numbers is available on PyPI:

```bash
 python -m pip install lakh-crore-numbers
```

## Documentation

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
