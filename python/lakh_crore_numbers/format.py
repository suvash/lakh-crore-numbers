import importlib
from functools import partial

from .parse import get_chunks
from .languages import nepali

LANGUAGES = {
  'nepali': nepali
}

def _format_chunk(lang_mod, chunk):
  (number, name) = chunk
  if name:
    result = f"{lang_mod.WORDS[number]} {lang_mod.AMOUNTS[name]}"
  else:
    result = lang_mod.WORDS[number]

  return result

def _format_num_char(lang_mod, num_char):
  return lang_mod.NUMBERS[num_char]

def _format_to_words(lang_mod, number):
  chunks = get_chunks(number)
  format_lang_chunk = partial(_format_chunk, lang_mod)

  return " ".join(map(format_lang_chunk, chunks))

def _format_to_numeral(lang_mod, number):
  format_lang_num_char = partial(_format_num_char, lang_mod)

  return "".join(map(format_lang_num_char, str(number)))

def format_to_nepali_words(number):
  return _format_to_words(LANGUAGES['nepali'], number)

def format_to_nepali_numeral(number):
  return _format_to_numeral(LANGUAGES['nepali'], number)
