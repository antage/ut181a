# ut181a

[![Documentation](https://docs.rs/ut181a/badge.svg)](https://docs.rs/ut181a) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)] [![Build Status](https://travis-ci.org/antage/ut181a.svg?branch=master)](https://travis-ci.org/antage/ut181a)

Uni-T UT181A digital multimeter (DMM) remote control library.
It supports USB connection only.

## Documentation

API documentation: https://docs.rs.ut181a

## Building

```
$ sudo apt-get install libusb-1.0-0-dev
$ cargo build
```

## Usage

``` rust
extern ut181a;

use ut181a::DmmEnumerator;

fn main() {
    // Don't use `unwrap()` in a production code!
    let enumerator = DmmEnumerator::new().unwrap();
    let mut dmm = enumerator.open_first().unwrap();

    dmm.monitor_on().unwrap();
    let measurement = dmm.read_measurement().unwrap();
    println!("Got measurement: {:?}", measurement);
    dmm.monitor_off().unwrap();
}
```

## License

This library licensed under the following:

* MIT License ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)
