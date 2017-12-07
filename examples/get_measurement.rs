extern crate hid;
extern crate ut181a;

use ut181a::{Dmm, Measurement};

fn run() -> Result<(), ut181a::Error> {
    let manager = hid::init()?;
    for device in manager.find(Some(0x10C4), Some(0xEA80)) {
        let mut dmm = Dmm::new(device.open()?)?;

        dmm.monitor_on()?;
        for _ in 1..10 {
            let m: Measurement = dmm.get_measurement()?;
            println!("{:?}", m);
        }
        dmm.monitor_off()?;

        break;
    }
    Ok(())
}

fn main() {
    match run() {
        Err(err) => {
            eprintln!("ERROR: {}", err);
        }
        _ => {}
    }
}
