mod errors;
mod languages;
mod parse;
mod types;

use errors::UnsupportedLargeNumberError;
use languages::nepali::Nepali;
use languages::Translatable;

pub fn format_to_nepali_numeral(number: u64) -> String {
    let nepali = Nepali::new();
    nepali.format_to_numeral(number)
}

pub fn format_to_nepali_words(number: u64) -> Result<String, UnsupportedLargeNumberError> {
    let nepali = Nepali::new();
    nepali.format_to_words(number)
}

#[cfg(test)]
mod tests {
    use super::*;
    use parse;

    #[test]
    fn test_format_to_nepali_words_ok_when_less_than_max_number() {
        assert_eq!(
            format_to_nepali_words(8359).unwrap(),
            "आठ हजार तीन सय उनन्साट्ठी"
        );
    }

    #[test]
    fn test_format_to_nepali_words_err_when_larger_than_max_number() {
        let input = parse::MAX_NUMBER + 1;

        assert_eq!(
            format_to_nepali_words(input).unwrap_err(),
            UnsupportedLargeNumberError {
                number: input,
                max_number: parse::MAX_NUMBER
            }
        );
    }
}
