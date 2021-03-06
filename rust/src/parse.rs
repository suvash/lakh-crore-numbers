use super::errors::UnsupportedLargeNumberError;
use super::types::Chunk;
use super::types::Scale;
use super::types::Unit;

use super::MAX_NUMBER;

static UNITS: [Scale; 10] = [
    Scale {
        unit: Unit::Shankha,
        amount: 1_00_00_00_00_00_00_00_000,
    },
    Scale {
        unit: Unit::Padma,
        amount: 1_00_00_00_00_00_00_000,
    },
    Scale {
        unit: Unit::Neel,
        amount: 1_00_00_00_00_00_000,
    },
    Scale {
        unit: Unit::Kharab,
        amount: 1_00_00_00_00_000,
    },
    Scale {
        unit: Unit::Arab,
        amount: 1_00_00_00_000,
    },
    Scale {
        unit: Unit::Crore,
        amount: 1_00_00_000,
    },
    Scale {
        unit: Unit::Lakh,
        amount: 1_00_000,
    },
    Scale {
        unit: Unit::Hajaar,
        amount: 1_000,
    },
    Scale {
        unit: Unit::Saya,
        amount: 100,
    },
    Scale {
        unit: Unit::None,
        amount: 1,
    },
];

pub fn chunks(number: u64) -> Result<Vec<Chunk>, UnsupportedLargeNumberError> {
    let chunks = match number {
        0 => Ok(vec![Chunk {
            amount: 0,
            unit: Unit::None,
        }]),
        1..=MAX_NUMBER => Ok(build_chunks(number, &UNITS)),
        _ => Err(UnsupportedLargeNumberError {
            number,
            max_number: MAX_NUMBER,
        }),
    };

    chunks
}

fn build_chunks(number: u64, scales: &[Scale; 10]) -> Vec<Chunk> {
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut chunk_number = number;

    for scale in scales.iter() {
        if chunk_number >= scale.amount {
            let quotient = (chunk_number / scale.amount) as u8;
            let remainder = chunk_number % scale.amount;

            chunks.push(Chunk {
                amount: quotient,
                unit: scale.unit,
            });

            chunk_number = remainder;
        }
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunks_err_when_larger_than_max_number() {
        let input = MAX_NUMBER + 1;
        assert_eq!(
            chunks(input).unwrap_err(),
            UnsupportedLargeNumberError {
                number: input,
                max_number: MAX_NUMBER
            }
        );
    }

    #[test]
    fn test_chunks_ok_when_less_than_max_number() {
        assert_eq!(
            chunks(0).unwrap(),
            vec![Chunk {
                amount: 0,
                unit: Unit::None
            }]
        );

        assert_eq!(
            chunks(13).unwrap(),
            vec![Chunk {
                amount: 13,
                unit: Unit::None
            }]
        );

        assert_eq!(
            chunks(908).unwrap(),
            vec![
                Chunk {
                    amount: 9,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 8,
                    unit: Unit::None
                }
            ]
        );
        assert_eq!(
            chunks(5_900).unwrap(),
            vec![
                Chunk {
                    amount: 5,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 9,
                    unit: Unit::Saya
                }
            ]
        );

        assert_eq!(
            chunks(10_246).unwrap(),
            vec![
                Chunk {
                    amount: 10,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 2,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 46,
                    unit: Unit::None
                }
            ]
        );

        assert_eq!(
            chunks(8_47_100).unwrap(),
            vec![
                Chunk {
                    amount: 8,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 47,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 1,
                    unit: Unit::Saya
                }
            ]
        );

        assert_eq!(
            chunks(84_00_712).unwrap(),
            vec![
                Chunk {
                    amount: 84,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 7,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 12,
                    unit: Unit::None
                }
            ]
        );

        assert_eq!(
            chunks(9_10_01_587).unwrap(),
            vec![
                Chunk {
                    amount: 9,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 10,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 1,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 5,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 87,
                    unit: Unit::None
                }
            ]
        );

        assert_eq!(
            chunks(48_02_87_694).unwrap(),
            vec![
                Chunk {
                    amount: 48,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 2,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 87,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 6,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 94,
                    unit: Unit::None
                },
            ]
        );

        assert_eq!(
            chunks(9_01_64_83_057).unwrap(),
            vec![
                Chunk {
                    amount: 9,
                    unit: Unit::Arab
                },
                Chunk {
                    amount: 1,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 64,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 83,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 57,
                    unit: Unit::None
                }
            ]
        );

        assert_eq!(
            chunks(30_27_42_04_793).unwrap(),
            vec![
                Chunk {
                    amount: 30,
                    unit: Unit::Arab
                },
                Chunk {
                    amount: 27,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 42,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 4,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 7,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 93,
                    unit: Unit::None
                }
            ]
        );

        assert_eq!(
            chunks(8_60_28_46_59_490).unwrap(),
            vec![
                Chunk {
                    amount: 8,
                    unit: Unit::Kharab
                },
                Chunk {
                    amount: 60,
                    unit: Unit::Arab
                },
                Chunk {
                    amount: 28,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 46,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 59,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 4,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 90,
                    unit: Unit::None
                }
            ]
        );
        assert_eq!(
            chunks(60_78_36_29_56_359).unwrap(),
            vec![
                Chunk {
                    amount: 60,
                    unit: Unit::Kharab
                },
                Chunk {
                    amount: 78,
                    unit: Unit::Arab
                },
                Chunk {
                    amount: 36,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 29,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 56,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 3,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 59,
                    unit: Unit::None
                }
            ]
        );

        assert_eq!(
            chunks(2_34_63_48_43_73_958).unwrap(),
            vec![
                Chunk {
                    amount: 2,
                    unit: Unit::Neel
                },
                Chunk {
                    amount: 34,
                    unit: Unit::Kharab
                },
                Chunk {
                    amount: 63,
                    unit: Unit::Arab
                },
                Chunk {
                    amount: 48,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 43,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 73,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 9,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 58,
                    unit: Unit::None
                }
            ]
        );

        assert_eq!(
            chunks(42_38_34_95_46_39_435).unwrap(),
            vec![
                Chunk {
                    amount: 42,
                    unit: Unit::Neel
                },
                Chunk {
                    amount: 38,
                    unit: Unit::Kharab
                },
                Chunk {
                    amount: 34,
                    unit: Unit::Arab
                },
                Chunk {
                    amount: 95,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 46,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 39,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 4,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 35,
                    unit: Unit::None
                }
            ]
        );

        assert_eq!(
            chunks(5_23_85_69_07_97_85_879).unwrap(),
            vec![
                Chunk {
                    amount: 5,
                    unit: Unit::Padma
                },
                Chunk {
                    amount: 23,
                    unit: Unit::Neel
                },
                Chunk {
                    amount: 85,
                    unit: Unit::Kharab
                },
                Chunk {
                    amount: 69,
                    unit: Unit::Arab
                },
                Chunk {
                    amount: 7,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 97,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 85,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 8,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 79,
                    unit: Unit::None
                }
            ]
        );

        assert_eq!(
            chunks(86_04_73_62_43_75_97_047).unwrap(),
            vec![
                Chunk {
                    amount: 86,
                    unit: Unit::Padma
                },
                Chunk {
                    amount: 4,
                    unit: Unit::Neel
                },
                Chunk {
                    amount: 73,
                    unit: Unit::Kharab
                },
                Chunk {
                    amount: 62,
                    unit: Unit::Arab
                },
                Chunk {
                    amount: 43,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 75,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 97,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 47,
                    unit: Unit::None
                }
            ]
        );

        assert_eq!(
            chunks(9_68_74_62_34_86_57_46_593).unwrap(),
            vec![
                Chunk {
                    amount: 9,
                    unit: Unit::Shankha
                },
                Chunk {
                    amount: 68,
                    unit: Unit::Padma
                },
                Chunk {
                    amount: 74,
                    unit: Unit::Neel
                },
                Chunk {
                    amount: 62,
                    unit: Unit::Kharab
                },
                Chunk {
                    amount: 34,
                    unit: Unit::Arab
                },
                Chunk {
                    amount: 86,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 57,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 46,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 5,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 93,
                    unit: Unit::None
                }
            ]
        );

        assert_eq!(
            chunks(82_05_95_79_52_68_50_73_935).unwrap(),
            vec![
                Chunk {
                    amount: 82,
                    unit: Unit::Shankha
                },
                Chunk {
                    amount: 5,
                    unit: Unit::Padma
                },
                Chunk {
                    amount: 95,
                    unit: Unit::Neel
                },
                Chunk {
                    amount: 79,
                    unit: Unit::Kharab
                },
                Chunk {
                    amount: 52,
                    unit: Unit::Arab
                },
                Chunk {
                    amount: 68,
                    unit: Unit::Crore
                },
                Chunk {
                    amount: 50,
                    unit: Unit::Lakh
                },
                Chunk {
                    amount: 73,
                    unit: Unit::Hajaar
                },
                Chunk {
                    amount: 9,
                    unit: Unit::Saya
                },
                Chunk {
                    amount: 35,
                    unit: Unit::None
                }
            ]
        );
    }
}
