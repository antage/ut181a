use std::fmt;
use std::result;

use error::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    VAC_Normal,
    VAC_Normal_Rel,
    VAC_Hz,
    VAC_Peak,
    VAC_LowPass,
    VAC_LowPass_Rel,
    VAC_dBV,
    VAC_dBV_Rel,
    VAC_dBm,
    VAC_dBm_Rel,

    mVAC_Normal,
    mVAC_Normal_Rel,
    mVAC_Hz,
    mVAC_Peak,
    mVAC_AC_DC,
    mVAC_AC_DC_Rel,

    VDC_Normal,
    VDC_Normal_Rel,
    VDC_AC_DC,
    VDC_AC_DC_Rel,
    VDC_Peak,

    mVDC_Normal,
    mVDC_Normal_Rel,
    mVDC_Peak,

    TempC_T1_T2,
    TempC_T1_T2_Rel,
    TempC_T2_T1,
    TempC_T2_T1_Rel,
    TempC_T1_T2_Diff,
    TempC_T2_T1_Diff,

    TempF_T1_T2,
    TempF_T1_T2_Rel,
    TempF_T2_T1,
    TempF_T2_T1_Rel,
    TempF_T1_T2_Diff,
    TempF_T2_T1_Diff,

    Resistance,
    Resistance_Rel,

    Beeper_Short,
    Beeper_Open,

    Admittance,
    Admittance_Rel,

    Diode_Normal,
    Diode_Alarm,

    Capacitance,
    Capacitance_Rel,

    Frequency,
    Frequency_Rel,

    DutyCycle,
    DutyCycle_Rel,

    PulseWidth,
    PulseWidth_Rel,

    uADC_Normal,
    uADC_Normal_Rel,
    uADC_AC_DC,
    uADC_AC_DC_Rel,
    uADC_Peak,

    uAAC_Normal,
    uAAC_Normal_Rel,
    uAAC_Hz,
    uAAC_Peak,

    mADC_Normal,
    mADC_Normal_Rel,
    mADC_AC_DC,
    mADC_AC_DC_Rel,
    mADC_Peak,

    mAAC_Normal,
    mAAC_Normal_Rel,
    mAAC_Hz,
    mAAC_Peak,

    ADC_Normal,
    ADC_Normal_Rel,
    ADC_AC_DC,
    ADC_AC_DC_Rel,
    ADC_Peak,

    AAC_Normal,
    AAC_Normal_Rel,
    AAC_Hz,
    AAC_Peak,
}

impl Mode {
    pub(crate) fn from_bin(data: &[u8]) -> Result<Mode> {
        use byteorder::{ByteOrder, LittleEndian};
        let m = LittleEndian::read_u16(data);
        match m {
            0x1111 => Ok(Mode::VAC_Normal),
            0x1112 => Ok(Mode::VAC_Normal_Rel),
            0x1121 => Ok(Mode::VAC_Hz),
            0x1131 => Ok(Mode::VAC_Peak),
            0x1141 => Ok(Mode::VAC_LowPass),
            0x1142 => Ok(Mode::VAC_LowPass_Rel),
            0x1151 => Ok(Mode::VAC_dBV),
            0x1152 => Ok(Mode::VAC_dBV_Rel),
            0x1161 => Ok(Mode::VAC_dBm),
            0x1162 => Ok(Mode::VAC_dBm_Rel),

            0x2111 => Ok(Mode::mVAC_Normal),
            0x2112 => Ok(Mode::mVAC_Normal_Rel),
            0x2121 => Ok(Mode::mVAC_Hz),
            0x2131 => Ok(Mode::mVAC_Peak),
            0x2141 => Ok(Mode::mVAC_AC_DC),
            0x2142 => Ok(Mode::mVAC_AC_DC_Rel),

            0x3111 => Ok(Mode::VDC_Normal),
            0x3112 => Ok(Mode::VDC_Normal_Rel),
            0x3121 => Ok(Mode::VDC_AC_DC),
            0x3122 => Ok(Mode::VDC_AC_DC_Rel),
            0x3131 => Ok(Mode::VDC_Peak),

            0x4111 => Ok(Mode::mVDC_Normal),
            0x4112 => Ok(Mode::mVDC_Normal_Rel),
            0x4121 => Ok(Mode::mVDC_Peak),

            0x4211 => Ok(Mode::TempC_T1_T2),
            0x4212 => Ok(Mode::TempC_T1_T2_Rel),
            0x4221 => Ok(Mode::TempC_T2_T1),
            0x4222 => Ok(Mode::TempC_T2_T1_Rel),
            0x4231 => Ok(Mode::TempC_T1_T2_Diff),
            0x4241 => Ok(Mode::TempC_T2_T1_Diff),

            0x4311 => Ok(Mode::TempF_T1_T2),
            0x4312 => Ok(Mode::TempF_T1_T2_Rel),
            0x4321 => Ok(Mode::TempF_T2_T1),
            0x4322 => Ok(Mode::TempF_T2_T1_Rel),
            0x4331 => Ok(Mode::TempF_T1_T2_Diff),
            0x4341 => Ok(Mode::TempF_T2_T1_Diff),

            0x5111 => Ok(Mode::Resistance),
            0x5112 => Ok(Mode::Resistance_Rel),

            0x5211 => Ok(Mode::Beeper_Short),
            0x5212 => Ok(Mode::Beeper_Open),

            0x5311 => Ok(Mode::Admittance),
            0x5312 => Ok(Mode::Admittance_Rel),

            0x6111 => Ok(Mode::Diode_Normal),
            0x6112 => Ok(Mode::Diode_Alarm),

            0x6211 => Ok(Mode::Capacitance),
            0x6212 => Ok(Mode::Capacitance_Rel),

            0x7111 => Ok(Mode::Frequency),
            0x7112 => Ok(Mode::Frequency_Rel),

            0x7211 => Ok(Mode::DutyCycle),
            0x7212 => Ok(Mode::DutyCycle_Rel),

            0x7311 => Ok(Mode::PulseWidth),
            0x7312 => Ok(Mode::PulseWidth_Rel),

            0x8111 => Ok(Mode::uADC_Normal),
            0x8112 => Ok(Mode::uADC_Normal_Rel),
            0x8121 => Ok(Mode::uADC_AC_DC),
            0x8122 => Ok(Mode::uADC_AC_DC_Rel),
            0x8131 => Ok(Mode::uADC_Peak),

            0x8211 => Ok(Mode::uAAC_Normal),
            0x8212 => Ok(Mode::uAAC_Normal_Rel),
            0x8221 => Ok(Mode::uAAC_Hz),
            0x8231 => Ok(Mode::uAAC_Peak),

            0x9111 => Ok(Mode::mADC_Normal),
            0x9112 => Ok(Mode::mADC_Normal_Rel),
            0x9121 => Ok(Mode::mADC_AC_DC),
            0x9122 => Ok(Mode::mADC_AC_DC_Rel),
            0x9131 => Ok(Mode::mADC_Peak),

            0x9211 => Ok(Mode::mAAC_Normal),
            0x9212 => Ok(Mode::mAAC_Normal_Rel),
            0x9221 => Ok(Mode::mAAC_Hz),
            0x9231 => Ok(Mode::mAAC_Peak),

            0xA111 => Ok(Mode::ADC_Normal),
            0xA112 => Ok(Mode::ADC_Normal_Rel),
            0xA121 => Ok(Mode::ADC_AC_DC),
            0xA122 => Ok(Mode::ADC_AC_DC_Rel),
            0xA131 => Ok(Mode::ADC_Peak),

            0xA211 => Ok(Mode::AAC_Normal),
            0xA212 => Ok(Mode::AAC_Normal_Rel),
            0xA221 => Ok(Mode::AAC_Hz),
            0xA231 => Ok(Mode::AAC_Peak),

            mode => Err(ErrorKind::UnknownMeasurementMode(mode).into()),
        }
    }

    pub(crate) fn to_bin(self) -> [u8; 2] {
        match self {
            Mode::VAC_Normal => [0x11, 0x11],
            Mode::VAC_Normal_Rel => [0x12, 0x11],
            Mode::VAC_Hz => [0x21, 0x11],
            Mode::VAC_Peak => [0x31, 0x11],
            Mode::VAC_LowPass => [0x41, 0x11],
            Mode::VAC_LowPass_Rel => [0x42, 0x11],
            Mode::VAC_dBV => [0x51, 0x11],
            Mode::VAC_dBV_Rel => [0x52, 0x11],
            Mode::VAC_dBm => [0x61, 0x11],
            Mode::VAC_dBm_Rel => [0x62, 0x11],

            Mode::mVAC_Normal => [0x11, 0x21],
            Mode::mVAC_Normal_Rel => [0x12, 0x21],
            Mode::mVAC_Hz => [0x21, 0x21],
            Mode::mVAC_Peak => [0x31, 0x21],
            Mode::mVAC_AC_DC => [0x41, 0x21],
            Mode::mVAC_AC_DC_Rel => [0x42, 0x21],

            Mode::VDC_Normal => [0x11, 0x31],
            Mode::VDC_Normal_Rel => [0x12, 0x31],
            Mode::VDC_AC_DC => [0x21, 0x31],
            Mode::VDC_AC_DC_Rel => [0x22, 0x31],
            Mode::VDC_Peak => [0x31, 0x31],

            Mode::mVDC_Normal => [0x11, 0x41],
            Mode::mVDC_Normal_Rel => [0x12, 0x41],
            Mode::mVDC_Peak => [0x21, 0x41],

            Mode::TempC_T1_T2 => [0x11, 0x42],
            Mode::TempC_T1_T2_Rel => [0x12, 0x42],
            Mode::TempC_T2_T1 => [0x21, 0x42],
            Mode::TempC_T2_T1_Rel => [0x22, 0x42],
            Mode::TempC_T1_T2_Diff => [0x31, 0x42],
            Mode::TempC_T2_T1_Diff => [0x41, 0x42],

            Mode::TempF_T1_T2 => [0x11, 0x43],
            Mode::TempF_T1_T2_Rel => [0x12, 0x43],
            Mode::TempF_T2_T1 => [0x21, 0x43],
            Mode::TempF_T2_T1_Rel => [0x22, 0x43],
            Mode::TempF_T1_T2_Diff => [0x31, 0x43],
            Mode::TempF_T2_T1_Diff => [0x41, 0x43],

            Mode::Resistance => [0x11, 0x51],
            Mode::Resistance_Rel => [0x12, 0x51],

            Mode::Beeper_Short => [0x11, 0x52],
            Mode::Beeper_Open => [0x12, 0x52],

            Mode::Admittance => [0x11, 0x53],
            Mode::Admittance_Rel => [0x12, 0x53],

            Mode::Diode_Normal => [0x11, 0x61],
            Mode::Diode_Alarm => [0x12, 0x61],

            Mode::Capacitance => [0x11, 0x62],
            Mode::Capacitance_Rel => [0x12, 0x62],

            Mode::Frequency => [0x11, 0x71],
            Mode::Frequency_Rel => [0x12, 0x71],

            Mode::DutyCycle => [0x11, 0x72],
            Mode::DutyCycle_Rel => [0x12, 0x72],

            Mode::PulseWidth => [0x11, 0x73],
            Mode::PulseWidth_Rel => [0x12, 0x73],

            Mode::uADC_Normal => [0x11, 0x81],
            Mode::uADC_Normal_Rel => [0x12, 0x81],
            Mode::uADC_AC_DC => [0x21, 0x81],
            Mode::uADC_AC_DC_Rel => [0x22, 0x81],
            Mode::uADC_Peak => [0x31, 0x81],

            Mode::uAAC_Normal => [0x11, 0x82],
            Mode::uAAC_Normal_Rel => [0x12, 0x82],
            Mode::uAAC_Hz => [0x21, 0x82],
            Mode::uAAC_Peak => [0x31, 0x82],

            Mode::mADC_Normal => [0x11, 0x91],
            Mode::mADC_Normal_Rel => [0x12, 0x91],
            Mode::mADC_AC_DC => [0x21, 0x91],
            Mode::mADC_AC_DC_Rel => [0x22, 0x91],
            Mode::mADC_Peak => [0x31, 0x91],

            Mode::mAAC_Normal => [0x11, 0x92],
            Mode::mAAC_Normal_Rel => [0x12, 0x92],
            Mode::mAAC_Hz => [0x21, 0x92],
            Mode::mAAC_Peak => [0x31, 0x92],

            Mode::ADC_Normal => [0x11, 0xA1],
            Mode::ADC_Normal_Rel => [0x12, 0xA1],
            Mode::ADC_AC_DC => [0x21, 0xA1],
            Mode::ADC_AC_DC_Rel => [0x22, 0xA1],
            Mode::ADC_Peak => [0x31, 0xA1],

            Mode::AAC_Normal => [0x11, 0xA2],
            Mode::AAC_Normal_Rel => [0x12, 0xA2],
            Mode::AAC_Hz => [0x21, 0xA2],
            Mode::AAC_Peak => [0x31, 0xA2],
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Mode::VAC_Normal => f.write_str("VAC"),
            Mode::VAC_Normal_Rel => f.write_str("VAC/Rel"),
            Mode::VAC_Hz => f.write_str("VAC/Hz"),
            Mode::VAC_Peak => f.write_str("VAC/Peak"),
            Mode::VAC_LowPass => f.write_str("VAC/Low Pass"),
            Mode::VAC_LowPass_Rel => f.write_str("VAC/Low Pass/Rel"),
            Mode::VAC_dBV => f.write_str("VAC/dBV"),
            Mode::VAC_dBV_Rel => f.write_str("VAC/dBV/Rel"),
            Mode::VAC_dBm => f.write_str("VAC/dBm"),
            Mode::VAC_dBm_Rel => f.write_str("VAC/dBm/Rel"),

            Mode::mVAC_Normal => f.write_str("mVAC"),
            Mode::mVAC_Normal_Rel => f.write_str("mVAC/Rel"),
            Mode::mVAC_Hz => f.write_str("mVAC/Hz"),
            Mode::mVAC_Peak => f.write_str("mVAC/Peak"),
            Mode::mVAC_AC_DC => f.write_str("mVAC/AC+DC"),
            Mode::mVAC_AC_DC_Rel => f.write_str("mVAC/AC+DC/Rel"),

            Mode::VDC_Normal => f.write_str("VDC"),
            Mode::VDC_Normal_Rel => f.write_str("VDC/Rel"),
            Mode::VDC_AC_DC => f.write_str("VDC/AC+DC"),
            Mode::VDC_AC_DC_Rel => f.write_str("VDC/AC+DC/Rel"),
            Mode::VDC_Peak => f.write_str("VDC/Peak"),

            Mode::mVDC_Normal => f.write_str("mVDC"),
            Mode::mVDC_Normal_Rel => f.write_str("mVDC/Rel"),
            Mode::mVDC_Peak => f.write_str("mVDC/Peak"),

            Mode::TempC_T1_T2 => f.write_str("Temp C/T1,T2"),
            Mode::TempC_T1_T2_Rel => f.write_str("Temp C/T1,T2/Rel"),
            Mode::TempC_T2_T1 => f.write_str("Temp C/T2,T1"),
            Mode::TempC_T2_T1_Rel => f.write_str("Temp C/T2,T1/Rel"),
            Mode::TempC_T1_T2_Diff => f.write_str("Temp C/T1-T2"),
            Mode::TempC_T2_T1_Diff => f.write_str("Temp C/T2-T1"),

            Mode::TempF_T1_T2 => f.write_str("Temp F/T1,T2"),
            Mode::TempF_T1_T2_Rel => f.write_str("Temp F/T1,T2/Rel"),
            Mode::TempF_T2_T1 => f.write_str("Temp F/T2,T1"),
            Mode::TempF_T2_T1_Rel => f.write_str("Temp F/T2,T1/Rel"),
            Mode::TempF_T1_T2_Diff => f.write_str("Temp F/T1-T2"),
            Mode::TempF_T2_T1_Diff => f.write_str("Temp F/T2-T1"),

            Mode::Resistance => f.write_str("Resistance"),
            Mode::Resistance_Rel => f.write_str("Resistance/Rel"),

            Mode::Beeper_Short => f.write_str("Beeper/Short"),
            Mode::Beeper_Open => f.write_str("Beeper/Open"),

            Mode::Admittance => f.write_str("Admittance"),
            Mode::Admittance_Rel => f.write_str("Admittance/Rel"),

            Mode::Diode_Normal => f.write_str("Diode"),
            Mode::Diode_Alarm => f.write_str("Diode/Alarm"),

            Mode::Capacitance => f.write_str("Capacitance"),
            Mode::Capacitance_Rel => f.write_str("Capacitance/Rel"),

            Mode::Frequency => f.write_str("Frequency"),
            Mode::Frequency_Rel => f.write_str("Frequency/Rel"),

            Mode::DutyCycle => f.write_str("Duty Cycle"),
            Mode::DutyCycle_Rel => f.write_str("Duty cycle/Rel"),

            Mode::PulseWidth => f.write_str("Pulse width"),
            Mode::PulseWidth_Rel => f.write_str("Pulse width/Rel"),

            Mode::uADC_Normal => f.write_str("uADC"),
            Mode::uADC_Normal_Rel => f.write_str("uADC/Rel"),
            Mode::uADC_AC_DC => f.write_str("uADC/AC+DC"),
            Mode::uADC_AC_DC_Rel => f.write_str("uADC/AC+DC/Rel"),
            Mode::uADC_Peak => f.write_str("uADC/Peak"),

            Mode::uAAC_Normal => f.write_str("uAAC"),
            Mode::uAAC_Normal_Rel => f.write_str("uAAC/Rel"),
            Mode::uAAC_Hz => f.write_str("uAAC/Hz"),
            Mode::uAAC_Peak => f.write_str("uAAC/Peak"),

            Mode::mADC_Normal => f.write_str("mADC"),
            Mode::mADC_Normal_Rel => f.write_str("mADC/Rel"),
            Mode::mADC_AC_DC => f.write_str("mADC/AC+DC"),
            Mode::mADC_AC_DC_Rel => f.write_str("mADC/AC+DC/Rel"),
            Mode::mADC_Peak => f.write_str("mADC/Peak"),

            Mode::mAAC_Normal => f.write_str("mAAC"),
            Mode::mAAC_Normal_Rel => f.write_str("mAAC/Rel"),
            Mode::mAAC_Hz => f.write_str("mAAC/Hz"),
            Mode::mAAC_Peak => f.write_str("mAAC/Peak"),

            Mode::ADC_Normal => f.write_str("ADC"),
            Mode::ADC_Normal_Rel => f.write_str("ADC/Rel"),
            Mode::ADC_AC_DC => f.write_str("ADC/AC+DC"),
            Mode::ADC_AC_DC_Rel => f.write_str("ADC/AC+DC/Rel"),
            Mode::ADC_Peak => f.write_str("ADC/Peak"),

            Mode::AAC_Normal => f.write_str("AAC"),
            Mode::AAC_Normal_Rel => f.write_str("AAC/Rel"),
            Mode::AAC_Hz => f.write_str("AAC/Hz"),
            Mode::AAC_Peak => f.write_str("AAC/Peak"),
        }
    }
}
