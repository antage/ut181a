#![recursion_limit="128"]

extern crate byteorder;
extern crate chrono;
extern crate cp211x_uart;
#[macro_use]
extern crate error_chain;
extern crate hid;
#[macro_use]
extern crate nom;

pub mod error;
use error::*;
pub use error::Error;

mod packet;
mod range;
mod mode;
mod unit;
mod value;
mod message;
mod measurement;
mod rec_info;
mod rec_data;
mod utils;

use std::time::{Duration, Instant};
use std::vec::Vec;

use byteorder::{ByteOrder, LittleEndian};
use chrono::NaiveDateTime;

use packet::Packet;
use message::Message;
pub use value::Value;
pub use measurement::Measurement;
pub use mode::Mode;
pub use range::Range;
pub use unit::{Unit, UnitExp};
pub use rec_info::RecordInfo;
pub use rec_data::RecordDataItem;

const RX_BUF_LENGTH: usize = 4096; // it should be 2.5KB at least
const WAIT_TIMEOUT: u64 = 5000; // 5 seconds

pub struct Dmm {
    uart: cp211x_uart::HidUart,
    rx_buf: Vec<u8>,
}

impl Dmm {
    pub fn new(handle: hid::Handle) -> Result<Dmm> {
        let mut uart = cp211x_uart::HidUart::new(handle)?;
        uart.set_read_timeout(Duration::from_millis(100));
        uart.set_write_timeout(Duration::from_millis(500));
        uart.set_config(&cp211x_uart::UartConfig {
            baud_rate: 9600,
            data_bits: cp211x_uart::DataBits::Bits8,
            stop_bits: cp211x_uart::StopBits::Short,
            parity: cp211x_uart::Parity::None,
            flow_control: cp211x_uart::FlowControl::None,
        })?;
        uart.flush_fifos(true, true)?;
        Ok(Dmm {
            uart,
            rx_buf: Vec::with_capacity(RX_BUF_LENGTH),
        })
    }

    /// Emulates 'Hold' button.
    pub fn toggle_hold(&mut self) -> Result<()> {
        let cmd = Packet::new(&[0x12, 0x5A]);
        self.uart
            .write(&cmd.frame())
            .chain_err(|| ErrorKind::CommandWrite("TOGGLE_HOLD"))?;

        self.wait_success()
    }

    /// Save current measurement in DMM memory.
    pub fn save(&mut self) -> Result<()> {
        let cmd = Packet::new(&[0x06]);
        self.uart
            .write(&cmd.frame())
            .chain_err(|| ErrorKind::CommandWrite("SAVE"))?;

        self.wait_success()
    }

    /// Get saved measurement count.
    pub fn save_count(&mut self) -> Result<u16> {
        let cmd = Packet::new(&[0x08]);
        self.uart
            .write(&cmd.frame())
            .chain_err(|| ErrorKind::CommandWrite("GET_SAVES_COUNT"))?;

        let reply = self.wait_reply(0x08)?;
        Ok(LittleEndian::read_u16(&reply))
    }

    /// Get saved measurement.
    pub fn read_save(&mut self, n: u16) -> Result<(NaiveDateTime, Measurement)> {
        let mut cmd: [u8; 3] = [0x07, 0x00, 0x00];
        LittleEndian::write_u16(&mut cmd[1..], n);
        let pkt = Packet::new(&cmd);

        self.uart
            .write(&pkt.frame())
            .chain_err(|| ErrorKind::CommandWrite("GET_SAVE"))?;

        let (datetime, measurement) = self.wait_save()?;

        Ok((datetime, measurement))
    }

    /// Delete saved measurement.
    ///
    /// `index` - save ID (1..0xFFFE).
    pub fn delete_save(&mut self, index: u16) -> Result<()> {
        let mut cmd: [u8; 3] = [0x09, 0x00, 0x00];
        LittleEndian::write_u16(&mut cmd[1..], index);
        let pkt = Packet::new(&cmd);

        self.uart
            .write(&pkt.frame())
            .chain_err(|| ErrorKind::CommandWrite("DELETE_SAVE"))?;

        self.wait_success()
    }

    /// Delete all saved measurements.
    pub fn delete_all_save(&mut self) -> Result<()> {
        self.delete_save(0xFFFF)
    }

    /// Turn on/off Min/Max mode.
    pub fn min_max_mode(&mut self, on: bool) -> Result<()> {
        let cmd = if on {
            Packet::new(&[0x04, 0x01])
        } else {
            Packet::new(&[0x04, 0x00])
        };
        self.uart
            .write(&cmd.frame())
            .chain_err(|| ErrorKind::CommandWrite("TOGGLE_MIN_MAX_MODE"))?;

        self.wait_success()
    }

    /// Set measuring range.
    ///
    /// Invalid step (`Range::Step8` in mVDC mode for example) switches DMM to next range.
    pub fn set_range(&mut self, range: Range) -> Result<()> {
        let b = range.to_bin();
        let cmd = Packet::new(&[0x02, b]);
        self.uart
            .write(&cmd.frame())
            .chain_err(|| ErrorKind::CommandWrite("SET_RANGE"))?;

        self.wait_success()
    }

    /// Set reference value in relative measuring mode.
    pub fn set_reference_value(&mut self, val: f32) -> Result<()> {
        let mut cmd: [u8; 5] = [0x03, 0x00, 0x00, 0x00, 0x00];
        LittleEndian::write_f32(&mut cmd[1..], val);
        let pkt = Packet::new(&cmd);
        self.uart
            .write(&pkt.frame())
            .chain_err(|| ErrorKind::CommandWrite("SET_REFERENCE"))?;

        self.wait_success()
    }

    /// Set mode and submode.
    pub fn set_mode(&mut self, mode: Mode) -> Result<()> {
        let mut cmd: [u8; 3] = [0x01, 0x00, 0x00];
        cmd[1..].copy_from_slice(&mode.to_bin()[..]);
        let pkt = Packet::new(&cmd);
        self.uart
            .write(&pkt.frame())
            .chain_err(|| ErrorKind::CommandWrite("SET_MODE"))?;

        self.wait_success()
    }

    /// Get record count.
    pub fn record_count(&mut self) -> Result<u16> {
        let cmd = Packet::new(&[0x0E]);
        self.uart
            .write(&cmd.frame())
            .chain_err(|| ErrorKind::CommandWrite("GET_RECORDS_COUNT"))?;

        let reply = self.wait_reply(0x0E)?;
        Ok(LittleEndian::read_u16(&reply))
    }

    /// Get record info.
    pub fn record_info(&mut self, i: u16) -> Result<RecordInfo> {
        let mut cmd: [u8; 3] = [0x0C, 0x00, 0x00];
        LittleEndian::write_u16(&mut cmd[1..], i);
        let pkt = Packet::new(&cmd);
        self.uart
            .write(&pkt.frame())
            .chain_err(|| ErrorKind::CommandWrite("GET_RECORD_INFO"))?;

        let reply = self.wait_record_info()?;
        Ok(reply)
    }

    /// Get record samples.
    /// `i` is record index (starting from 1).
    pub fn record_data(&mut self, i: u16) -> Result<Vec<RecordDataItem>> {
        let mut offset = 1;
        let info = self.record_info(i)?;
        let mut items: Vec<RecordDataItem> = Vec::new();
        let mut cmd: [u8; 7] = [0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        LittleEndian::write_u16(&mut cmd[1..], i);
        loop {
            LittleEndian::write_u32(&mut cmd[3..], offset as u32);
            let pkt = Packet::new(&cmd);
            self.uart
                .write(&pkt.frame())
                .chain_err(|| ErrorKind::CommandWrite("GET_RECORD_DATA"))?;

            let raw_items = self.wait_record_data()?;
            let raw_items_count = raw_items.len();
            if raw_items_count == 0 {
                return Ok(items);
            }
            items.extend(raw_items.into_iter().map(
                |item: message::RawRecordDataItem| {
                    RecordDataItem {
                        value: Value {
                            overload_neg: item.overload_neg,
                            overload_pos: item.overload_pos,
                            value: item.value,
                            precision: Some(item.precision),
                            unit: info.unit,
                        },
                        timestamp: item.timestamp,
                    }
                },
            ));
            offset += raw_items_count;
        }
    }

    /// Turn on monitoring mode.
    pub fn monitor_on(&mut self) -> Result<()> {
        let pkt = Packet::new(&[0x05, 0x01]);
        self.uart
            .write(&pkt.frame())
            .chain_err(|| ErrorKind::CommandWrite("MONITOR_ON"))?;

        self.wait_measurement()?;
        Ok(())
    }

    /// Turn off monitoring mode.
    pub fn monitor_off(&mut self) -> Result<()> {
        let pkt = Packet::new(&[0x05, 0x00]);
        self.uart
            .write(&pkt.frame())
            .chain_err(|| ErrorKind::CommandWrite("MONITOR_OFF"))?;

        self.wait_success_or_measurement()?;
        Ok(())
    }

    /// Returns first message from DMM.
    ///
    /// This function blocks thread until to read a message
    /// or exceeds `WAIT_TIMEOUT` duration.
    pub(crate) fn read_message(&mut self) -> Result<Message> {
        use std::cmp::max;
        let now = Instant::now();
        loop {
            if now.elapsed() > Duration::from_millis(WAIT_TIMEOUT) {
                return Err(ErrorKind::WaitTimeout.into());
            }
            match Message::from_bin(&self.rx_buf)? {
                None => {
                    let mut buf: [u8; 64] = [0; 64];
                    let read = self.uart.read(&mut buf)?;
                    self.rx_buf.extend_from_slice(&buf[0..read]);
                }
                Some((msg, consumed)) => {
                    let new_len = self.rx_buf.len() - consumed;
                    let mut new_buf = Vec::with_capacity(max(new_len, RX_BUF_LENGTH));
                    new_buf.resize(new_len, 0);
                    new_buf.copy_from_slice(&self.rx_buf[consumed..]);
                    self.rx_buf = new_buf;
                    return Ok(msg);
                }
            }
        }
    }

    /// Returns measurement from DMM.
    ///
    /// This function blocks thread until to read a message
    /// or exceeds `WAIT_TIMEOUT` duration.
    pub fn read_measurement(&mut self) -> Result<Measurement> {
        self.wait_measurement()
    }

    fn wait_reply(&mut self, cmd: u8) -> Result<Vec<u8>> {
        let now = Instant::now();
        loop {
            if now.elapsed() > Duration::from_millis(WAIT_TIMEOUT) {
                return Err(ErrorKind::WaitTimeout.into());
            }
            let msg = self.read_message()?;
            if let Message::Error = msg {
                return Err(ErrorKind::CommandError.into());
            }
            if let Message::Reply(data) = msg {
                if data[0] == cmd {
                    let mut v = Vec::with_capacity(data.len() - 1);
                    v.resize(data.len() - 1, 0);
                    v.copy_from_slice(&data[1..]);
                    return Ok(v);
                }
            }
        }
    }

    fn wait_measurement(&mut self) -> Result<Measurement> {
        let now = Instant::now();
        loop {
            if now.elapsed() > Duration::from_millis(WAIT_TIMEOUT) {
                return Err(ErrorKind::WaitTimeout.into());
            }
            let msg = self.read_message()?;
            if let Message::Error = msg {
                return Err(ErrorKind::CommandError.into());
            }
            if let Message::Measurement(measurement) = msg {
                return Ok(measurement);
            }
        }
    }

    fn wait_success(&mut self) -> Result<()> {
        let now = Instant::now();
        loop {
            if now.elapsed() > Duration::from_millis(WAIT_TIMEOUT) {
                return Err(ErrorKind::WaitTimeout.into());
            }
            let msg = self.read_message()?;
            if let Message::Error = msg {
                return Err(ErrorKind::CommandError.into());
            }
            if let Message::Success = msg {
                return Ok(());
            }
        }
    }

    fn wait_success_or_measurement(&mut self) -> Result<()> {
        let now = Instant::now();
        loop {
            if now.elapsed() > Duration::from_millis(WAIT_TIMEOUT) {
                return Err(ErrorKind::WaitTimeout.into());
            }
            let msg = self.read_message()?;
            if let Message::Error = msg {
                return Err(ErrorKind::CommandError.into());
            }
            if let Message::Success = msg {
                return Ok(());
            }
            if let Message::Measurement(_) = msg {
                return Ok(());
            }
        }
    }

    fn wait_save(&mut self) -> Result<(NaiveDateTime, Measurement)> {
        let now = Instant::now();
        loop {
            if now.elapsed() > Duration::from_millis(WAIT_TIMEOUT) {
                return Err(ErrorKind::WaitTimeout.into());
            }
            let msg = self.read_message()?;
            if let Message::Error = msg {
                return Err(ErrorKind::CommandError.into());
            }
            if let Message::Save(datetime, measurement) = msg {
                return Ok((datetime, measurement));
            }
        }
    }

    fn wait_record_info(&mut self) -> Result<RecordInfo> {
        let now = Instant::now();
        loop {
            if now.elapsed() > Duration::from_millis(WAIT_TIMEOUT) {
                return Err(ErrorKind::WaitTimeout.into());
            }
            let msg = self.read_message()?;
            if let Message::Error = msg {
                return Err(ErrorKind::CommandError.into());
            }
            if let Message::RecordInfo(info) = msg {
                return Ok(info);
            }
        }
    }

    fn wait_record_data(&mut self) -> Result<Vec<message::RawRecordDataItem>> {
        let now = Instant::now();
        loop {
            if now.elapsed() > Duration::from_millis(WAIT_TIMEOUT) {
                return Err(ErrorKind::WaitTimeout.into());
            }
            let msg = self.read_message()?;
            if let Message::Error = msg {
                return Err(ErrorKind::CommandError.into());
            }
            if let Message::RecordData(items) = msg {
                return Ok(items);
            }
        }
    }
}
