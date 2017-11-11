use std::time::Duration;

use byteorder::{ByteOrder, LittleEndian};

use error::*;
use mode::Mode;
use range::Range;
use value::Value;
use unit::UnitExp;

#[derive(Clone, Copy, Debug)]
enum MeasurementKind {
    Normal,
    Relative,
    MinMax,
    Peak,
}

#[derive(Clone, Copy, Debug)]
pub enum Measurement {
    Normal {
        mode: Mode,
        is_holded: bool,
        is_auto_range: bool,
        range: Range,
        main: Value,
        aux1: Option<Value>,
        aux2: Option<Value>,
        fast: Option<Value>,
    },
    Relative {
        mode: Mode,
        is_holded: bool,
        is_auto_range: bool,
        range: Range,
        relative: Value,
        reference: Value,
        measurement: Value,
        fast: Option<Value>,
    },
    MinMax {
        mode: Mode,
        is_holded: bool,
        is_auto_range: bool,
        range: Range,
        main: Value,
        max: Value,
        max_time: Duration,
        average: Value,
        average_time: Duration,
        min: Value,
        min_time: Duration,
    },
    Peak {
        mode: Mode,
        is_holded: bool,
        is_auto_range: bool,
        range: Range,
        min: Value,
        max: Value,
    },
}

fn read_duration(data: &[u8]) -> Duration {
    Duration::from_secs(u64::from(LittleEndian::read_u32(data)))
}

impl Measurement {
    pub(crate) fn from_bin(data: &[u8]) -> Result<Measurement> {
        let kind = match data[0] & 0x70 {
            0x00 => MeasurementKind::Normal,
            0x10 => MeasurementKind::Relative,
            0x20 => MeasurementKind::MinMax,
            0x40 => MeasurementKind::Peak,
            _ => return Err(ErrorKind::UnknownMeasurementKind(data[0]).into()),
        };

        let is_fast = (data[0] & 0x08) != 0;
        let is_holded = (data[0] & 0x80) != 0;
        let is_auto_range = data[1] == 1;
        let mode = Mode::from_bin(&data[2..])?;
        let range = Range::from_bin(&data[4..])?;

        match kind {
            MeasurementKind::Normal => {
                let mut offset = 5;
                let main = Value::from_bin_with_precision_and_unit(&data[offset..])?;
                offset += 13;

                let aux1 = if data[0] & 0x02 != 0 {
                    let val = Value::from_bin_with_precision_and_unit(&data[offset..])?;
                    offset += 13;
                    Some(val)
                } else {
                    None
                };

                let aux2 = if data[0] & 0x04 != 0 {
                    let val = Value::from_bin_with_precision_and_unit(&data[offset..])?;
                    offset += 13;
                    Some(val)
                } else {
                    None
                };

                let fast = if is_fast {
                    let val = Value::from_bin_fast(&data[offset..])?;
                    Some(val)
                } else {
                    None
                };

                Ok(Measurement::Normal {
                    mode,
                    is_holded,
                    is_auto_range,
                    range,
                    main,
                    aux1,
                    aux2,
                    fast,
                })
            }
            MeasurementKind::Relative => {
                let relative = Value::from_bin_with_precision_and_unit(&data[5..])?;
                let reference = Value::from_bin_with_precision_and_unit(&data[18..])?;
                let measurement = Value::from_bin_with_precision_and_unit(&data[31..])?;

                let fast = if is_fast {
                    let fast = Value::from_bin_fast(&data[44..])?;
                    Some(fast)
                } else {
                    None
                };

                Ok(Measurement::Relative {
                    mode,
                    is_holded,
                    is_auto_range,
                    range,
                    relative,
                    reference,
                    measurement,
                    fast,
                })
            }
            MeasurementKind::MinMax => {
                let unit = UnitExp::from_bin(&data[37..])?;
                let main = Value::from_bin_with_precision(&data[5..], unit)?;
                let max = Value::from_bin_with_precision(&data[10..], unit)?;
                let max_time = read_duration(&data[15..]);
                let average = Value::from_bin_with_precision(&data[19..], unit)?;
                let average_time = read_duration(&data[24..]);
                let min = Value::from_bin_with_precision(&data[28..], unit)?;
                let min_time = read_duration(&data[33..]);

                Ok(Measurement::MinMax {
                    mode,
                    is_holded,
                    is_auto_range,
                    range,
                    main,
                    max,
                    max_time,
                    average,
                    average_time,
                    min,
                    min_time,
                })
            }
            MeasurementKind::Peak => {
                let max = Value::from_bin_with_precision_and_unit(&data[5..])?;
                let min = Value::from_bin_with_precision_and_unit(&data[18..])?;

                Ok(Measurement::Peak {
                    mode,
                    is_holded,
                    is_auto_range,
                    range,
                    min,
                    max,
                })
            }
        }
    }
}
