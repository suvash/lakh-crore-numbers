import unittest

from lakh_crore_numbers.format import format_to_nepali_words as fnw
from lakh_crore_numbers.format import format_to_nepali_numeral as fnn


class TestFormat(unittest.TestCase):
  def test_format_to_nepali_numeral(self):
    assert fnn(34789) == "३४७८९"

  def test_format_to_nepali_words(self):
    assert fnw(0) == "सुन्ना"
    assert fnw(92486540) == "नौ करोड चौबीस लाख छयासी हजार पाँच सय चालीस"

if __name__ == '__main__':
    unittest.main()
