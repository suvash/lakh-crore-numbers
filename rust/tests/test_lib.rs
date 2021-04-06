use lakh_crore_numbers as lcn;

#[test]
fn test_format_to_nepali_words_ok_when_less_than_max_number() {
    assert_eq!(
        lcn::format_to_nepali_words(8359).unwrap(),
        "आठ हजार तीन सय उनन्साट्ठी"
    );
}

#[test]
fn test_format_to_nepali_words_err_when_larger_than_max_number() {
    let input = lcn::MAX_NUMBER + 1;

    assert_eq!(
        lcn::format_to_nepali_words(input).unwrap_err(),
        lcn::errors::UnsupportedLargeNumberError {
            number: input,
            max_number: lcn::MAX_NUMBER
        }
    );
}
