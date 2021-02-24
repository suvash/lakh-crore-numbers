import unittest

from lakh_crore_numbers.parse import get_chunks
from lakh_crore_numbers.errors import UnsupportedLargeNumberError

class TestChunks(unittest.TestCase):
  def test_get_chunks(self):
    assert get_chunks(0) == [(0, None)]
    assert get_chunks(13) == [(13, None)]
    assert get_chunks(910) == [(9, 'saya'), (10, None)]
    assert get_chunks(5300) == [(5, 'hajaar'), (3, 'saya')]
    assert get_chunks(10245) == [(10, 'hajaar'), (2, 'saya'), (45, None)]
    assert get_chunks(858200) == [(8, 'lakh'), (58, 'hajaar'), (2, 'saya')]
    assert get_chunks(6400371) == [(64, 'lakh'), (3, 'saya'), (71, None)]
    assert get_chunks(91001587) == [(9, 'crore'), (10, 'lakh'), (1, 'hajaar'), (5, 'saya'), (87, None)]
    assert get_chunks(480287694) == [(48, 'crore'), (2, 'lakh'), (87, 'hajaar'), (6, 'saya'), (94, None)]
    assert get_chunks(9016483057) == [(9, 'arab'), (1, 'crore'), (64, 'lakh'), (83, 'hajaar'), (57, None)]
    assert get_chunks(30274204793) == [(30, 'arab'), (27, 'crore'), (42, 'lakh'), (4, 'hajaar'), (7, 'saya'), (93, None)]
    assert get_chunks(860284659490) == [(8, 'kharab'), (60, 'arab'), (28, 'crore'), (46, 'lakh'), (59, 'hajaar'), (4, 'saya'), (90, None)]
    assert get_chunks(6078362956359) == [(60, 'kharab'), (78, 'arab'), (36, 'crore'), (29, 'lakh'), (56, 'hajaar'), (3, 'saya'), (59, None)]
    assert get_chunks(23463484373958) == [(2, 'neel'), (34, 'kharab'), (63, 'arab'), (48, 'crore'), (43, 'lakh'), (73, 'hajaar'), (9, 'saya'), (58, None)]
    assert get_chunks(423834954639435) == [(42, 'neel'), (38, 'kharab'), (34, 'arab'), (95, 'crore'), (46, 'lakh'), (39, 'hajaar'), (4, 'saya'), (35, None)]
    assert get_chunks(5238569079785879) == [(5, 'padma'), (23, 'neel'), (85, 'kharab'), (69, 'arab'), (7, 'crore'), (97, 'lakh'), (85, 'hajaar'), (8, 'saya'), (79, None)]
    assert get_chunks(86047362437597047) == [(86, 'padma'), (4, 'neel'), (73, 'kharab'), (62, 'arab'), (43, 'crore'), (75, 'lakh'), (97, 'hajaar'), (47, None)]
    assert get_chunks(968746234865746593) == [(9, 'shankha'), (68, 'padma'), (74, 'neel'), (62, 'kharab'), (34, 'arab'), (86, 'crore'), (57, 'lakh'), (46, 'hajaar'), (5, 'saya'), (93, None)]
    assert get_chunks(8205957952685073935) == [(82, 'shankha'), (5, 'padma'), (95, 'neel'), (79, 'kharab'), (52, 'arab'), (68, 'crore'), (50, 'lakh'), (73, 'hajaar'), (9, 'saya'), (35, None)]

  def test_get_chunks_raises_error(self):
    with self.assertRaises(UnsupportedLargeNumberError):
      get_chunks(1_00_00_00_00_00_00_00_00_000)


if __name__ == '__main__':
    unittest.main()
