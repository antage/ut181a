use std::fmt;

use byteorder::{ByteOrder, LittleEndian};

use error::*;
use unit::UnitExp;

/// Measured value.
#[derive(Clone, Debug)]
pub struct Value {
    /// Is `value` under negative limit?
    pub overload_neg: bool,

    /// Is `value` over positive limit?
    pub overload_pos: bool,

    /// Measured value.
    pub value: f32,

    /// A number of digits after decimal point.
    pub precision: Option<usize>,

    /// Measurement unit and exponent.
    pub unit: UnitExp,
}

impl Value {
    pub(crate) fn from_bin_with_precision_and_unit(data: &[u8]) -> Result<Value> {
        let prc_rdy = data[4];
        let overload_neg = (prc_rdy & 0x0E) == 0x02;
        let overload_pos = (prc_rdy & 0x01) == 0x01;
        let precision = usize::from(prc_rdy >> 4);
        let unit = UnitExp::from_bin(&data[5..])?;

        let value: f32 = LittleEndian::read_f32(data);

        Ok(Value {
            overload_neg,
            overload_pos,
            value,
            precision: Some(precision),
            unit,
        })
    }

    pub(crate) fn from_bin_with_precision(data: &[u8], unit: UnitExp) -> Result<Value> {
        let prc_rdy = data[4];
        let overload_neg = (prc_rdy & 0x0E) == 0x02;
        let overload_pos = (prc_rdy & 0x01) == 0x01;
        let precision = usize::from(prc_rdy >> 4);

        let value: f32 = LittleEndian::read_f32(data);

        Ok(Value {
            overload_neg,
            overload_pos,
            value,
            precision: Some(precision),
            unit,
        })
    }

    pub(crate) fn from_bin_fast(data: &[u8]) -> Result<Value> {
        let unit = UnitExp::from_bin(&data[4..])?;

        let value: f32 = LittleEndian::read_f32(data);

        Ok(Value {
            overload_neg: false,
            overload_pos: false,
            value,
            precision: None,
            unit,
        })
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.overload_neg {
            return write!(f, "-OL");
        }
        if self.overload_pos {
            return write!(f, "OL");
        }
        if let Some(prc) = self.precision {
            write!(f, "{:.*}", prc, self.value)?;
        } else {
            write!(f, "{}", self.value)?;
        }
        write!(f, " {}", self.unit)
    }
}
