# ut181a

[![Documentation](https://docs.rs/ut181a/badge.svg)](https://docs.rs/ut181a) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) [![Build Status](https://travis-ci.org/antage/ut181a.svg?branch=master)](https://travis-ci.org/antage/ut181a)

Uni-T UT181A digital multimeter (DMM) remote control library.
It supports USB connection only.

## Documentation

API documentation is [here](https://docs.rs/ut181a).

## Building

### Linux

```
$ sudo apt-get install libudev-dev libhidapi-dev
$ cargo build
```

## Usage

``` rust
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
```

## License

This library licensed under the following:

* MIT License ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)
