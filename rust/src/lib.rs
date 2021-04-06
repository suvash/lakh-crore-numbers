pub mod errors;
mod languages;
mod parse;
mod types;

use errors::UnsupportedLargeNumberError;
use languages::nepali::Nepali;
use languages::Translatable;

/// This is the maximum integer number that will be
///  processed by this library when formatting to words
pub const MAX_NUMBER: u64 = 99_99_99_99_99_99_99_99_999;

/// Formats an integer (u64) into a Nepali numeral string
///
/// # Examples
///
/// ```
/// use lakh_crore_numbers as lcn;
///
/// assert_eq!(lcn::format_to_nepali_numeral(295678), "२९५६७८");
/// ```
pub fn format_to_nepali_numeral(number: u64) -> String {
    let nepali = Nepali::new();
    nepali.format_to_numeral(number)
}

/// Formats an integer (u64) into a Result type with Nepali word string
/// if the integer is less than the MAX_NUMBER, else UnsupportedLargeNumberError
///
/// # Examples
///
/// ```
/// use lakh_crore_numbers as lcn;
///
/// assert_eq!(
///   lcn::format_to_nepali_words(8359),
///   Ok(String::from("आठ हजार तीन सय उनन्साट्ठी")));
/// ```
///
/// ```
/// use lakh_crore_numbers as lcn;
/// use lakh_crore_numbers::errors::UnsupportedLargeNumberError;
///
/// let maximum = lcn::MAX_NUMBER;
/// let input = maximum + 1;
/// assert_eq!(
///   lcn::format_to_nepali_words(input),
///   Err(UnsupportedLargeNumberError{number: input, max_number: maximum}));
/// ```
pub fn format_to_nepali_words(number: u64) -> Result<String, UnsupportedLargeNumberError> {
    let nepali = Nepali::new();
    nepali.format_to_words(number)
}
