import unittest

from lakh_crore_numbers.parse import get_chunks

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

  def test_get_chunks_raises_error(self):
    with self.assertRaises(Exception):
      get_chunks(99_99_99_999)

if __name__ == '__main__':
    unittest.main()
