extern crate hex;
extern crate image;
extern crate qrcode;
extern crate strum;

use qrcode::render::svg;
use qrcode::{Color, EcLevel, QrCode};

#[derive(PartialEq, Debug)]
pub struct QRCodeOptions {
    pub ec_level: ECLevel,
}

#[derive(PartialEq, Debug, Display, EnumString, Copy, Clone)]
pub enum ECLevel {
    L = 0,
    M = 1,
    Q = 2,
    H = 3,
}

impl From<ECLevel> for EcLevel {
    fn from(ec: ECLevel) -> Self {
        match ec {
            ECLevel::L => EcLevel::L,
            ECLevel::M => EcLevel::M,
            ECLevel::Q => EcLevel::Q,
            ECLevel::H => EcLevel::H,
        }
    }
}

const DEFAULT_OPTIONS: QRCodeOptions = QRCodeOptions { ec_level: ECLevel::M };

pub struct QRCode<'a> {
    opts: &'a QRCodeOptions,
}

impl<'a> QRCode<'a> {
    pub fn new(opts: &'a QRCodeOptions) -> Self {
        QRCode { opts }
    }

    pub fn default() -> Self {
        Self::new(QRCode::default_options())
    }

    pub fn default_options() -> &'static QRCodeOptions {
        &DEFAULT_OPTIONS
    }

    pub fn encode_to_vec(&self, data: &[u8]) -> Vec<Color> {
        self.encoder(data).to_colors()
    }

    fn encoder(&self, data: &[u8]) -> QrCode {
        QrCode::with_error_correction_level(data, EcLevel::from(self.opts.ec_level)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let default_options = QRCode::default_options();
        assert_eq!(default_options, &DEFAULT_OPTIONS);
    }

    #[test]
    fn test_default_ctor() {
        let code = QRCode::default();
        assert_eq!(code.opts, &DEFAULT_OPTIONS);
    }

    macro_rules! encode_to_vec_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (options, message, expected) = $value;
                let qr = QRCode::new(&options);
                let encoded = qr.encode_to_vec(message.as_bytes());
                let compact = encoded.iter().fold(String::new(), |acc, x| {
                    let v = match x {
                        Color::Dark => "0",
                        Color::Light => "1",
                    };

                    acc + v
                });
                assert_eq!(compact, expected);
            }
        )*
        }
    }

    encode_to_vec_tests! {
    test_encode_to_vec_0: (QRCodeOptions {ec_level: ECLevel::L}, "test",
        "000000011011010000000011111010110110111110010001011011110100010010001010110110100010010001011100010100010011111010001010111110000000010101010000000111111111100011111111000001000011001010101011000101101011010010111001010000101100001100101110011111000011100101011000101100101111111110000000111000000000010011010010101011111011010000110011010001010101011010101010001010101011010011010001010100101101011011111010101111001011000000010100101101001"),
    test_encode_to_vec_1: (QRCodeOptions {ec_level: ECLevel::M}, "test2",
        "000000011010110000000011111011010010111110010001010010110100010010001010000110100010010001010111010100010011111010000110111110000000010101010000000111111110111111111111010000011100110000011000101111010000110010010010010001010010001101100100010000110011111001000011011010101111111110001011011000000000011000101100101011111010101111000001010001010100101100101010001010000000010011010001010111010011011011111011100000001011000000010011011010101"),
    test_encode_to_vec_3: (QRCodeOptions {ec_level: ECLevel::Q}, "test3",
        "000000010101010000000011111010100110111110010001010001010100010010001010000110100010010001010101010100010011111011110110111110000000010101010000000111111110000011111111100101001011010100000111001110110110111100101110001110011100000000000110001110111101101010011110010100100111111110011101010110000000010000100010100011111011011001001111010001010110000010100010001011101110011101010001010101011101010011111010001110100101000000011101010100100"),
    test_encode_to_vec_4: (QRCodeOptions {ec_level: ECLevel::H}, "test4",
        "000000011010010000000011111011011110111110010001010110010100010010001010110010100010010001011001110100010011111011101110111110000000010101010000000111111111110011111111111001001010111110011011001111100001000011011001010111100011000101111100001111001011101100010110111001110111111110001100111011000000010100100110111011111011001011110000010001010110110101100010001010001000101011010001011010110000000011111011001111111000000000011110001100111"),
    }
}
