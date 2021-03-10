from .errors import UnsupportedLargeNumberError

MAX_NUMBER = 99_99_99_99_99_99_99_99_999

UNIT_AMOUNTS = [
  ('shankha', 1_00_00_00_00_00_00_00_000),
  ('padma',      1_00_00_00_00_00_00_000),
  ('neel',          1_00_00_00_00_00_000),
  ('kharab',           1_00_00_00_00_000),
  ('arab',                1_00_00_00_000),
  ('crore',                  1_00_00_000),
  ('lakh',                      1_00_000),
  ('hajaar',                       1_000),
  ('saya',                           100),
  (None,                               1),
]

def get_chunks(number):
  result = None
  if number > MAX_NUMBER:
    raise UnsupportedLargeNumberError(number, MAX_NUMBER)
  elif number == 0:
    result = [(0, None)]
  else:
    result = list(_chunkator(number, UNIT_AMOUNTS))

  return result

def _chunkator(number, unit_amounts):
  chunk_number = number
  for (unit, unit_amount) in unit_amounts:
    if chunk_number >= unit_amount:
      (quotient, remainder) = divmod(chunk_number, unit_amount)
      yield (quotient, unit)
      chunk_number = remainder
