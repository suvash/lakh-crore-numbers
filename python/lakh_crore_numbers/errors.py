class Error(Exception):
  pass

class UnsupportedLargeNumberError(Error):
  def __init__(self, number, max_number):
    self.number = number
    self.max_number = max_number

  def __str__(self):
    return f"Numbers larger than {self.max_number} not supported : {self.number}"
