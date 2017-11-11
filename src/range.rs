use error::*;

/// Measuring range.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Range {
    /// Auto range
    Auto,

    /// 60 mV/6 V/600 uA/60 mA/600 Ohm/60 Hz/6 nF
    Step1,

    /// 600 mV/60 V/6000 uA/600 mA/6 KOhm/600 Hz/60 nF
    Step2,

    /// 600V/60 KOhm/6 KHz/600 nF
    Step3,

    /// 1000 V/600 KOhm/60 KHz/6 uF
    Step4,

    /// 6 MOhm/600 KHz/60 uF
    Step5,

    /// 60 MOhm/6 MHz/600 uF
    Step6,

    /// 60 MHz/6 mF
    Step7,

    /// 60 mF
    Step8,
}

impl Range {
    pub(crate) fn from_bin(data: &[u8]) -> Result<Range> {
        match data[0] {
            0x00 => Ok(Range::Auto),
            0x01 => Ok(Range::Step1),
            0x02 => Ok(Range::Step2),
            0x03 => Ok(Range::Step3),
            0x04 => Ok(Range::Step4),
            0x05 => Ok(Range::Step5),
            0x06 => Ok(Range::Step6),
            0x07 => Ok(Range::Step7),
            0x08 => Ok(Range::Step8),
            b => Err(ErrorKind::UnknownMeasurementRange(b).into()),
        }
    }

    pub(crate) fn to_bin(self) -> u8 {
        match self {
            Range::Auto => 0x00,
            Range::Step1 => 0x01,
            Range::Step2 => 0x02,
            Range::Step3 => 0x03,
            Range::Step4 => 0x04,
            Range::Step5 => 0x05,
            Range::Step6 => 0x06,
            Range::Step7 => 0x07,
            Range::Step8 => 0x08,
        }
    }
}
