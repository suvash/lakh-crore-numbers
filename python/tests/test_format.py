import unittest

from lakh_crore_numbers.format import format_to_nepali_words as fnw
from lakh_crore_numbers.format import format_to_nepali_numeral as fnn


class TestFormat(unittest.TestCase):
  def test_format_to_nepali_numeral(self):
    assert fnn(34789) == "३४७८९"
    assert fnn(93539583458393) == "९३५३९५८३४५८३९३"

  def test_format_to_nepali_words(self):
    assert fnw(0) == "सुन्ना"
    assert fnw(4800) == "चार हजार आठ सय"
    assert fnw(74838) == "चौहत्तर हजार आठ सय अठतीस"
    assert fnw(673000) == "छ लाख त्रिहत्तर हजार"
    assert fnw(92486540) == "नौ करोड चौबीस लाख छयासी हजार पाँच सय चालीस"
    assert fnw(956000010) == "पन्चानब्बे करोड साठी लाख दस"
    assert fnw(724524629489) == "सात खर्ब चौबीस अर्ब बाउन्न करोड छयालीस लाख उनन्तीस हजार चार सय उनान्नब्बे"
    assert fnw(7480008380000) == "चौहत्तर खर्ब असी अर्ब त्रियासी लाख असी हजार"
    assert fnw(98345022845553579) == "अन्ठानब्बे पद्म चौँतीस नील पचास खर्ब बाइस अर्ब चौरासी करोड पचपन्न लाख त्रिपन्न हजार पाँच सय उनासी"
    assert fnw(243000000013700003) == "दुई शंख त्रिचालीस पद्म एक करोड सैँतीस लाख तीन"
    assert fnw(7454053303403530240) == "चौहत्तर शंख चवन्न पद्म पाँच नील तेत्तीस खर्ब तीन अर्ब चालीस करोड पैँतीस लाख तीस हजार दुई सय चालीस"

if __name__ == '__main__':
    unittest.main()
