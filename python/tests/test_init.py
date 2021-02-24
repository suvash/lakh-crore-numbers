import unittest

from lakh_crore_numbers import format_to_nepali_words as fnw
from lakh_crore_numbers import format_to_nepali_numeral as fnn
from lakh_crore_numbers.format import format_to_nepali_words as ffnw
from lakh_crore_numbers.format import format_to_nepali_numeral as ffnn


class TestInit(unittest.TestCase):
  def test_format_to_nepali_numeral_is_a_reexport(self):
    assert fnn == ffnn

  def test_format_to_nepali_words_is_a_reexport(self):
    assert fnw == ffnw

if __name__ == '__main__':
    unittest.main()
