mod nepali;
use nepali::Nepali;

use crate::types::Unit;
use std::collections::HashMap;

trait Translatable {
    fn name(&self) -> &str;
    fn words(&self, number: u8) -> &'static str;
    fn amounts(&self, unit: Unit) -> &'static str;
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
            vec![String::from("nepali")],
            available().keys().cloned().collect::<Vec<String>>()
        );
    }
}
