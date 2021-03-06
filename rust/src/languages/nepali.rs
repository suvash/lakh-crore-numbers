use super::Translatable;
use crate::types::Unit;

#[derive(Debug, PartialEq, Eq)]
pub struct Nepali {
    pub name: String,
}

impl Nepali {
    pub fn new() -> Nepali {
        Nepali {
            ..Default::default()
        }
    }
}

impl Default for Nepali {
    fn default() -> Nepali {
        Nepali {
            name: "nepali".to_string(),
        }
    }
}

impl Translatable for Nepali {
    fn name(&self) -> &str {
        &self.name
    }

    fn numbers(&self, numchar: char) -> &'static str {
        match numchar {
            '0' => "०",
            '1' => "१",
            '2' => "२",
            '3' => "३",
            '4' => "४",
            '5' => "५",
            '6' => "६",
            '7' => "७",
            '8' => "८",
            '9' => "९",
            _ => "",
        }
    }

    fn words(&self, number: u8) -> &'static str {
        match number {
            0 => "सुन्ना",
            1 => "एक",
            2 => "दुई",
            3 => "तीन",
            4 => "चार",
            5 => "पाँच",
            6 => "छ",
            7 => "सात",
            8 => "आठ",
            9 => "नौ",
            10 => "दस",
            11 => "एघार",
            12 => "बाह्र",
            13 => "तेह्र",
            14 => "चौध",
            15 => "पन्ध्र",
            16 => "सोह्र",
            17 => "सत्र",
            18 => "अठार",
            19 => "उन्नाइस",
            20 => "बीस",
            21 => "एक्काइस",
            22 => "बाइस",
            23 => "तेइस",
            24 => "चौबीस",
            25 => "पच्चीस",
            26 => "छब्बीस",
            27 => "सत्ताइस",
            28 => "अठ्ठाइस",
            29 => "उनन्तीस",
            30 => "तीस",
            31 => "एकतीस",
            32 => "बत्तीस",
            33 => "तेत्तीस",
            34 => "चौँतीस",
            35 => "पैँतीस",
            36 => "छत्तीस",
            37 => "सैँतीस",
            38 => "अठतीस",
            39 => "उनन्चालीस",
            40 => "चालीस",
            41 => "एकचालीस",
            42 => "बयालीस",
            43 => "त्रिचालीस",
            44 => "चवालीस",
            45 => "पैँतालीस",
            46 => "छयालीस",
            47 => "सतचालीस",
            48 => "अठचालीस",
            49 => "उनन्चास",
            50 => "पचास",
            51 => "एकाउन्न",
            52 => "बाउन्न",
            53 => "त्रिपन्न",
            54 => "चवन्न",
            55 => "पचपन्न",
            56 => "छपन्न",
            57 => "सन्ताउन्न",
            58 => "अन्ठाउन्न",
            59 => "उनन्साट्ठी",
            60 => "साठी",
            61 => "एकसट्ठी",
            62 => "बैसट्ठी",
            63 => "त्रिसट्ठी",
            64 => "चौँसट्ठी",
            65 => "पैँसट्ठी",
            66 => "छैसट्ठी",
            67 => "सतसट्ठी",
            68 => "अठसट्ठी",
            69 => "उनन्सत्तरी",
            70 => "सत्तरी",
            71 => "एकहत्तर",
            72 => "बहत्तर",
            73 => "त्रिहत्तर",
            74 => "चौहत्तर",
            75 => "पचहत्तर",
            76 => "छयहत्तर",
            77 => "सतहत्तर",
            78 => "अठहत्तर",
            79 => "उनासी",
            80 => "असी",
            81 => "एकासी",
            82 => "बयासी",
            83 => "त्रियासी",
            84 => "चौरासी",
            85 => "पचासी",
            86 => "छयासी",
            87 => "सतासी",
            88 => "अठासी",
            89 => "उनान्नब्बे",
            90 => "नब्बे",
            91 => "एकानब्बे",
            92 => "बयानब्बे",
            93 => "त्रियानब्बे",
            94 => "चौरानब्बे",
            95 => "पन्चानब्बे",
            96 => "छयानब्बे",
            97 => "सन्तानब्बे",
            98 => "अन्ठानब्बे",
            99 => "उनान्सय",
            _ => "",
        }
    }

    fn amounts(&self, unit: Unit) -> &'static str {
        match unit {
            Unit::Shankha => "शंख",
            Unit::Padma => "पद्म",
            Unit::Neel => "नील",
            Unit::Kharab => "खर्ब",
            Unit::Arab => "अर्ब",
            Unit::Crore => "करोड",
            Unit::Lakh => "लाख",
            Unit::Hajaar => "हजार",
            Unit::Saya => "सय",
            Unit::None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let lang = Nepali::new();
        assert_eq!("nepali", lang.name());
    }

    #[test]
    fn test_numbers() {
        let lang = Nepali::new();
        assert_eq!("२", lang.numbers('2'));
    }

    #[test]
    fn test_words() {
        let lang = Nepali::new();
        assert_eq!("अठतीस", lang.words(38));
    }

    #[test]
    fn test_amounts() {
        let lang = Nepali::new();
        assert_eq!("करोड", lang.amounts(Unit::Crore));
    }
}
