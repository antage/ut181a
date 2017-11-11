use chrono::{NaiveDate, NaiveDateTime};

use error::*;

pub(crate) fn read_stringz<'a>(data: &'a [u8]) -> String {
    let zestr: Vec<u8> = data.into_iter()
        .map(|b| *b)
        .take_while(|b| *b != 0)
        .collect();
    String::from_utf8_lossy(&zestr).into_owned()
}

/// Read `DateTime` from bytes.
pub(crate) fn read_datetime(data: &[u8]) -> Result<NaiveDateTime> {
    let year = 2000 + (i32::from(data[0]) & 0x3F);
    let month = (u32::from(data[0]) >> 6) | ((u32::from(data[1]) & 0x03) << 2);
    let day = u32::from(data[1] >> 2) & 0x1F;

    let hour = (u32::from(data[1]) >> 7) | ((u32::from(data[2]) & 0x0F) << 1);
    let minute = (u32::from(data[2]) >> 4) | ((u32::from(data[3]) & 0x03) << 4);
    let second = u32::from(data[3]) >> 2;

    match NaiveDate::from_ymd_opt(year, month, day) {
        None => Err(ErrorKind::InvalidDateTime(year, month, day, hour, minute, second).into()),
        Some(date) => match date.and_hms_opt(hour, minute, second) {
            None => Err(ErrorKind::InvalidDateTime(year, month, day, hour, minute, second).into()),
            Some(datetime) => Ok(datetime),
        },
    }
}
