mod nepali;
use crate::types::Unit;

trait Translate {
    fn words(&self, number: u8) -> &'static str;
    fn amounts(&self, unit: Unit) -> &'static str;
}
