pub mod errors;
mod languages;
mod parse;
mod types;

use errors::UnsupportedLargeNumberError;
use languages::nepali::Nepali;
use languages::Translatable;

pub const MAX_NUMBER: u64 = 99_99_99_99_99_99_99_99_999;

pub fn format_to_nepali_numeral(number: u64) -> String {
    let nepali = Nepali::new();
    nepali.format_to_numeral(number)
}

pub fn format_to_nepali_words(number: u64) -> Result<String, UnsupportedLargeNumberError> {
    let nepali = Nepali::new();
    nepali.format_to_words(number)
}
