from .errors import UnsupportedLargeNumberError

MAX_NUMBER = 99_99_99_99_99_99_99_99_999

def get_chunks(number):
  result = None
  if number > MAX_NUMBER:
    raise UnsupportedLargeNumberError(number, MAX_NUMBER)
  elif number == 0:
    result = [(0, None)]
  else:
    result = _get_chunks(number)

  return result

def _get_chunks(number):
  if number >= 1_00_00_00_00_00_00_00_000:
    (shankha, rest) = divmod(number, 1_00_00_00_00_00_00_00_000)
    result = [(shankha, 'shankha')] + _get_chunks(rest)
  elif number >= 1_00_00_00_00_00_00_000:
    (padma, rest) = divmod(number, 1_00_00_00_00_00_00_000)
    result = [(padma, 'padma')] + _get_chunks(rest)
  elif number >= 1_00_00_00_00_00_000:
    (neel, rest) = divmod(number, 1_00_00_00_00_00_000)
    result = [(neel, 'neel')] + _get_chunks(rest)
  elif number >= 1_00_00_00_00_000:
    (kharab, rest) = divmod(number, 1_00_00_00_00_000)
    result = [(kharab, 'kharab')] + _get_chunks(rest)
  elif number >= 1_00_00_00_000:
    (arab, rest) = divmod(number, 1_00_00_00_000)
    result = [(arab, 'arab')] + _get_chunks(rest)
  elif number >= 1_00_00_000:
    (crore, rest) = divmod(number, 1_00_00_000)
    result = [(crore, 'crore')] + _get_chunks(rest)
  elif number >= 1_00_000:
    (lakh, rest) = divmod(number, 1_00_000)
    result = [(lakh, 'lakh')] + _get_chunks(rest)
  elif number >= 1_000:
    (hajaar, rest) = divmod(number, 1_000)
    result = [(hajaar, 'hajaar')] + _get_chunks(rest)
  elif number >= 100:
    (saya, rest) = divmod(number, 100)
    result = [(saya, 'saya')] + _get_chunks(rest)
  elif number >= 1:
    result = [(number, None)]
  else:
    result = []

  return result
