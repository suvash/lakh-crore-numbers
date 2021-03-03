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
    result = _get_chunks(number, iter(UNIT_AMOUNTS))

  return result

def _get_chunks(number, unit_amounts_iter):
  unit_amount = next(unit_amounts_iter, None)

  if unit_amount is None:
    return []

  else:
    (unit, unit_amount) = unit_amount
    if number >= unit_amount:
      (quotient, remainder) = divmod(number, unit_amount)
      result = [(quotient, unit)] + _get_chunks(remainder, unit_amounts_iter)
    else:
      result = _get_chunks(number, unit_amounts_iter)

    return result
