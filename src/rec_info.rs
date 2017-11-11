use std::time::Duration;

use byteorder::{ByteOrder, LittleEndian};
use chrono::NaiveDateTime;

use error::*;
use value::Value;
use unit::UnitExp;
use utils::{read_stringz, read_datetime};

#[derive(Clone, Debug)]
pub struct RecordInfo {
    /// Name of the record.
    pub name: String,

    /// Unit and exponent.
    pub unit: UnitExp,

    /// Interval.
    pub interval: Duration,

    /// Duration.
    pub duration: Duration,

    /// Sample count.
    pub sample_count: u32,

    /// Maximum value.
    pub max: Value,

    /// Average value.
    pub average: Value,

    /// Minimum value.
    pub min: Value,

    /// Start date/time.
    pub start: NaiveDateTime,
}

impl RecordInfo {
    pub(crate) fn from_bin(data: &[u8]) -> Result<RecordInfo> {
        let name = read_stringz(&data[0..]);
        let unit = UnitExp::from_bin(&data[11..])?;
        let interval = LittleEndian::read_u16(&data[19..]);
        let duration = LittleEndian::read_u32(&data[21..]);
        let sample_count = LittleEndian::read_u32(&data[25..]);

        let max = Value::from_bin_with_precision(&data[29..], unit)?;
        let average = Value::from_bin_with_precision(&data[34..], unit)?;
        let min = Value::from_bin_with_precision(&data[39..], unit)?;

        let start = read_datetime(&data[44..])?;

        Ok(RecordInfo {
            name,
            unit,
            interval: Duration::from_secs(u64::from(interval)),
            duration: Duration::from_secs(u64::from(duration)),
            sample_count,
            max,
            average,
            min,
            start,
        })
    }
}
