use std::fmt;

use error::*;
use utils::read_stringz;

/// Measurement unit.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    /// Volts, direct current
    VDC,

    /// Volts, alternating current
    VAC,

    /// Volts, AC+DC
    VAcDc,

    /// Ampers, direct current
    ADC,

    /// Ampers, alternating current
    AAC,

    /// Ampers, AC+DC
    AAcDc,

    /// Celsius degrees
    Celsius,

    /// Fahrenheit degrees
    Fahrenheit,

    /// Farad
    F,

    /// Hertz
    Hz,

    /// seconds
    s,

    /// Percents
    Percent,

    /// Siemens
    S,

    /// Ohms
    Ohm,

    /// Decibels referenced to 1 milliwatt
    dBm,

    /// Decibels referenced to 1 Volt
    dBV,
}

#[derive(Clone, Copy, Debug)]
pub struct UnitExp {
    pub unit: Unit,
    pub exponent: isize,
}

impl UnitExp {
    /// Returns unit and decimal exponent.
    pub(crate) fn from_bin(data: &[u8]) -> Result<UnitExp> {
        match read_stringz(data).as_ref() {
            "mVDC" => Ok(UnitExp {
                unit: Unit::VDC,
                exponent: -3,
            }),
            "VDC" => Ok(UnitExp {
                unit: Unit::VDC,
                exponent: 0,
            }),
            "V" => Ok(UnitExp {
                unit: Unit::VDC,
                exponent: 0,
            }),
            "mVAC" => Ok(UnitExp {
                unit: Unit::VAC,
                exponent: -3,
            }),
            "VAC" => Ok(UnitExp {
                unit: Unit::VAC,
                exponent: 0,
            }),
            "mVac+dc" => Ok(UnitExp {
                unit: Unit::VAcDc,
                exponent: -3,
            }),
            "Vac+dc" => Ok(UnitExp {
                unit: Unit::VAcDc,
                exponent: 0,
            }),
            "uADC" => Ok(UnitExp {
                unit: Unit::ADC,
                exponent: -6,
            }),
            "mADC" => Ok(UnitExp {
                unit: Unit::ADC,
                exponent: -3,
            }),
            "ADC" => Ok(UnitExp {
                unit: Unit::ADC,
                exponent: 0,
            }),
            "uAAC" => Ok(UnitExp {
                unit: Unit::AAC,
                exponent: -6,
            }),
            "mAAC" => Ok(UnitExp {
                unit: Unit::AAC,
                exponent: -3,
            }),
            "AAC" => Ok(UnitExp {
                unit: Unit::AAC,
                exponent: 0,
            }),
            "uAac+dc" => Ok(UnitExp {
                unit: Unit::AAcDc,
                exponent: -6,
            }),
            "mAac+dc" => Ok(UnitExp {
                unit: Unit::AAcDc,
                exponent: -3,
            }),
            "Aac+dc" => Ok(UnitExp {
                unit: Unit::AAcDc,
                exponent: 0,
            }),
            "\u{FFFD}C" => Ok(UnitExp {
                unit: Unit::Celsius,
                exponent: 0,
            }),
            "\u{FFFD}F" => Ok(UnitExp {
                unit: Unit::Fahrenheit,
                exponent: 0,
            }),
            "Hz" => Ok(UnitExp {
                unit: Unit::Hz,
                exponent: 0,
            }),
            "kHz" => Ok(UnitExp {
                unit: Unit::Hz,
                exponent: 3,
            }),
            "MHz" => Ok(UnitExp {
                unit: Unit::Hz,
                exponent: 6,
            }),
            "ms" => Ok(UnitExp {
                unit: Unit::s,
                exponent: -3,
            }),
            "%" => Ok(UnitExp {
                unit: Unit::Percent,
                exponent: 0,
            }),
            "nS" => Ok(UnitExp {
                unit: Unit::S,
                exponent: -9,
            }),
            "~" => Ok(UnitExp {
                unit: Unit::Ohm,
                exponent: 0,
            }),
            "k~" => Ok(UnitExp {
                unit: Unit::Ohm,
                exponent: 3,
            }),
            "M~" => Ok(UnitExp {
                unit: Unit::Ohm,
                exponent: 6,
            }),
            "dBm" => Ok(UnitExp {
                unit: Unit::dBm,
                exponent: 0,
            }),
            "dBV" => Ok(UnitExp {
                unit: Unit::dBV,
                exponent: 0,
            }),
            "nF" => Ok(UnitExp {
                unit: Unit::F,
                exponent: -9,
            }),
            "uF" => Ok(UnitExp {
                unit: Unit::F,
                exponent: -6,
            }),
            "mF" => Ok(UnitExp {
                unit: Unit::F,
                exponent: -3,
            }),
            unt => Err(ErrorKind::UnknownMeasurementUnit(unt.into()).into()),
        }
    }
}

impl fmt::Display for UnitExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = match self.exponent {
            -12 => "p",
            -9 => "n",
            -6 => "u",
            -3 => "m",
            0 => "",
            3 => "k",
            6 => "M",
            9 => "G",
            _ => return Err(fmt::Error),
        };
        let unit = match self.unit {
            Unit::VDC => "VDC",
            Unit::VAC => "VAC",
            Unit::VAcDc => "Vac+dc",
            Unit::ADC => "ADC",
            Unit::AAC => "AAC",
            Unit::AAcDc => "Aac+dc",
            Unit::Celsius => "C",
            Unit::Fahrenheit => "F",
            Unit::F => "F",
            Unit::Hz => "Hz",
            Unit::s => "s",
            Unit::Percent => "%",
            Unit::S => "S",
            Unit::Ohm => "Ohm",
            Unit::dBm => "dBm",
            Unit::dBV => "dBV",
        };

        write!(f, "{}{}", prefix, unit)
    }
}
