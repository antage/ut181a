use byteorder::{ByteOrder, LittleEndian};
use nom::Endianness;

use chrono::NaiveDateTime;

use error::*;
use measurement::Measurement;
use rec_info::RecordInfo;
use packet::Packet;
use utils::read_datetime;

named!(message<&[u8], (&[u8], u16)>,
    do_parse!(
        take_until_and_consume!(&[0xAB, 0xCD][..]) >>
        len: u16!(Endianness::Little) >>
        data: take!(len - 2) >>
        chksum: u16!(Endianness::Little) >>
        (data, chksum)
    )
);

#[derive(Clone, Debug)]
pub(crate) enum Message {
    Success,
    Error,
    Measurement(Measurement),
    Save(NaiveDateTime, Measurement),
    Reply(Vec<u8>),
    RecordInfo(RecordInfo),
    RecordData(Vec<RawRecordDataItem>),
}

#[derive(Clone, Debug)]
pub(crate) struct RawRecordDataItem {
    pub(crate) overload_neg: bool,
    pub(crate) overload_pos: bool,
    pub(crate) value: f32,
    pub(crate) precision: usize,
    pub(crate) timestamp: NaiveDateTime,
}

impl Message {
    pub(crate) fn new(data: &[u8]) -> Result<Message> {
        match data[0] {
            0x01 => {
                let errno = LittleEndian::read_u16(&data[1..]);
                match errno {
                    0x4B4F => Ok(Message::Success),
                    0x5245 => Ok(Message::Error),
                    code => Err(ErrorKind::UnknownReplyCode(code).into()),
                }
            }
            0x02 => Ok(Message::Measurement(Measurement::from_bin(&data[1..])?)),
            0x03 => {
                let dt = read_datetime(&data[1..])?;
                let measurement = Measurement::from_bin(&data[5..])?;
                Ok(Message::Save(dt, measurement))
            }
            0x04 => Ok(Message::RecordInfo(RecordInfo::from_bin(&data[1..])?)),
            0x05 => {
                let count = usize::from(data[1]);
                let mut items = Vec::with_capacity(count);
                for i in 0..count {
                    let offset = 2 + i * 9;
                    let overload_neg = (data[offset + 4] & 0x02) != 0;
                    let overload_pos = (data[offset + 4] & 0x01) != 0;
                    let value = LittleEndian::read_f32(&data[offset..]);
                    let precision = usize::from(data[offset + 4] >> 4);
                    let timestamp = read_datetime(&data[(offset + 5)..])?;
                    items.push(RawRecordDataItem {
                        overload_neg,
                        overload_pos,
                        value,
                        precision,
                        timestamp,
                    });
                }
                Ok(Message::RecordData(items))
            }
            0x72 => {
                let mut vec = Vec::with_capacity(data.len() - 1);
                vec.resize(data.len() - 1, 0);
                vec.copy_from_slice(&data[1..]);
                Ok(Message::Reply(vec))
            }
            fmt => Err(ErrorKind::UnknownMessageFormat(fmt).into()),
        }
    }

    /// Return a message and a number of consumed bytes.
    pub(crate) fn from_bin(data: &[u8]) -> Result<Option<(Message, usize)>> {
        use nom::IResult;
        let mut offset = 0;

        if data.len() < 6 {
            return Ok(None);
        }

        loop {
            match message(&data[offset..]) {
                IResult::Incomplete(_) => return Ok(None),
                IResult::Error(err) => {
                    if err == ::nom::ErrorKind::TakeUntilAndConsume {
                        // We can't find 0xCDAB in the buffer yet.
                        return Ok(None);
                    }
                    panic!("Parser fatal error: {:?}", err);
                }
                IResult::Done(rest, (msg_data, chksum)) => {
                    let msg = Message::new(msg_data)?;
                    let consumed = data.len() - rest.len();
                    if Packet::new(msg_data).checksum() == chksum {
                        return Ok(Some((msg, consumed)));
                    } else {
                        offset = consumed;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    fn parse_data(data: &[u8], expected_data: &[u8], expected_chksum: u16, expected_rest: &[u8]) {
        match super::message(data) {
            IResult::Done(rest, (data, chksum)) => {
                assert_eq!(data, expected_data);
                assert_eq!(chksum, expected_chksum);
                assert_eq!(rest, expected_rest);
            }
            _ => panic!("message(data) should return Done"),
        }
    }

    #[test]
    fn test_parse1() {
        let data = [0xAB, 0xCD, 0x03, 0x00, 0x05, 0x12, 0x21];
        parse_data(&data, &[0x05], 0x2112, &[]);
    }

    #[test]
    fn test_parse_with_prefix() {
        let data = [
            0x00,
            0x00,
            0xAB,
            0x00,
            0xCD,
            0xAB,
            0xCD,
            0x03,
            0x00,
            0x05,
            0x12,
            0x21,
        ];
        parse_data(&data, &[0x05], 0x2112, &[]);
    }

    #[test]
    fn test_parse_with_suffix() {
        let data = [0xAB, 0xCD, 0x03, 0x00, 0x05, 0x12, 0x21, 0x00, 0x00];
        parse_data(&data, &[0x05], 0x2112, &[0x00, 0x00]);
    }
}
