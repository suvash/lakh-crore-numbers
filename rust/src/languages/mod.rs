mod nepali;
use nepali::Nepali;

use crate::errors::UnsupportedLargeNumberError;
use crate::parse;
use crate::types::{Chunk, Unit};
use std::collections::HashMap;

trait Translatable {
    fn name(&self) -> &str;
    fn numbers(&self, numchar: char) -> &'static str;
    fn words(&self, number: u8) -> &'static str;
    fn amounts(&self, unit: Unit) -> &'static str;

    fn format_chunk(&self, chunk: Chunk) -> String {
        let word = self.words(chunk.amount);
        let amount = self.amounts(chunk.unit);
        match chunk.unit {
            Unit::None => format!("{}", word),
            _ => format!("{} {}", word, amount),
        }
    }

    fn format_to_numeral(&self, number: u64) -> String {
        number
            .to_string()
            .chars()
            .map(|x| self.numbers(x))
            .collect::<String>()
    }

    fn format_to_words(&self, number: u64) -> Result<String, UnsupportedLargeNumberError> {
        let chunks = parse::get_chunks(number)?;
        let result = chunks
            .into_iter()
            .map(|x| self.format_chunk(x))
            .collect::<Vec<String>>()
            .join(" ");

        Ok(result)
    }
}

fn available() -> HashMap<String, Box<dyn Translatable>> {
    let mut languages: HashMap<String, Box<dyn Translatable>> = HashMap::new();

    let nepali = Nepali::new();
    languages.insert(nepali.name.clone(), Box::new(nepali));

    languages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available() {
        assert_eq!(1, available().len());
        assert_eq!(
            available().keys().cloned().collect::<Vec<String>>(),
            vec![String::from("nepali")],
        );
    }

    #[test]
    fn test_translatable_format_chunk() {
        let nepali = Nepali::new();
        assert_eq!(
            nepali.format_chunk(Chunk {
                amount: 23,
                unit: Unit::Padma
            }),
            "तेइस पद्म"
        );
        assert_eq!(
            nepali.format_chunk(Chunk {
                amount: 39,
                unit: Unit::None
            }),
            "उनन्चालीस"
        );
    }

    #[test]
    fn test_translatable_format_to_numeral() {
        let nepali = Nepali::new();
        assert_eq!(nepali.format_to_numeral(234), "२३४");
    }

    #[test]
    fn test_translatable_format_to_words_ok_when_less_than_max_number() {
        let nepali = Nepali::new();
        assert_eq!(nepali.format_to_words(234).unwrap(), "दुई सय चौँतीस");
    }

    #[test]
    fn test_translatable_format_to_words_err_when_larger_than_max_number() {
        let nepali = Nepali::new();
        let input = parse::MAX_NUMBER + 1;

        assert_eq!(
            nepali.format_to_words(input).unwrap_err(),
            UnsupportedLargeNumberError {
                number: input,
                max_number: parse::MAX_NUMBER
            }
        );
    }
}
