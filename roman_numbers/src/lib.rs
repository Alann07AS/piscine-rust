use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            1       => Self::I,
            5       => Self::V,
            10      => Self::X,
            50      => Self::L,
            100     => Self::C,
            500     => Self::D,
            1000    => Self::M,
            _       => Self::Nulla
        }
    }
}

// impl From<u32> for RomanNumber {
//     fn from(value: u32) -> Self {
//         let mut result: RomanNumber = RomanNumber(vec![]);
//         // let mut value = value;
//         let div = &[1000, 500, 100, 50, 10, 5, 1];
//         for (i, _) in div.iter().enumerate() {
//             let mut i2 = value/div[i];
//             let dif = div[i] - value;
//             if i2 > 3 {
//                 result.0.push(RomanDigit::from(div[i-1]));
//                 i2 = value / (div[i-1] / div[i]*i2)
//             }
//             for _ in 0..i2 {
//                 result.0.push(RomanDigit::from(div[i]))
//             }
//         }
//         result
//     }
// }

// while value != 0 {
    // if value / div[i_div] == 0 {
        // i_div += 1;
        // continue;
    // }
    // result.0.push()
// }


impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        let mut number = value;
        let mut digits = vec![];

        // Traitement des milliers
        let thousands = number / 1000;
        for _ in 0..thousands {
            digits.push(RomanDigit::M);
        }
        number %= 1000;

        // Traitement des centaines
        let hundreds = number / 100;
        match hundreds {
            4 => {
                digits.push(RomanDigit::C);
                digits.push(RomanDigit::D);
            }
            9 => {
                digits.push(RomanDigit::C);
                digits.push(RomanDigit::M);
            }
            _ => {
                if hundreds >= 5 {
                    digits.push(RomanDigit::D);
                }
                for _ in 0..(hundreds % 5) {
                    digits.push(RomanDigit::C);
                }
            }
        }
        number %= 100;

        // Traitement des dizaines
        let tens = number / 10;
        match tens {
            4 => {
                digits.push(RomanDigit::X);
                digits.push(RomanDigit::L);
            }
            9 => {
                digits.push(RomanDigit::X);
                digits.push(RomanDigit::C);
            }
            _ => {
                if tens >= 5 {
                    digits.push(RomanDigit::L);
                }
                for _ in 0..(tens % 5) {
                    digits.push(RomanDigit::X);
                }
            }
        }
        number %= 10;

        // Traitement des unités
        match number {
            4 => {
                digits.push(RomanDigit::I);
                digits.push(RomanDigit::V);
            }
            9 => {
                digits.push(RomanDigit::I);
                digits.push(RomanDigit::X);
            }
            _ => {
                if number >= 5 {
                    digits.push(RomanDigit::V);
                }
                for _ in 0..(number % 5) {
                    digits.push(RomanDigit::I);
                }
            }
        }

        RomanNumber(digits)
    }
}