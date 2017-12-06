# ut181a

[![Documentation](https://docs.rs/ut181a/badge.svg)](https://docs.rs/ut181a) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)] [![Build Status](https://travis-ci.org/antage/ut181a.svg?branch=master)](https://travis-ci.org/antage/ut181a)

Uni-T UT181A digital multimeter (DMM) remote control library.
It supports USB connection only.

## Documentation

API documentation: https://docs.rs.ut181a

## Building

### Linux

```
$ sudo apt-get install libudev-dev libhidapi-dev
$ cargo build
```

### Windows

Requirements:

* Rust target `*-pc-windows-msvc`: VisualStudio 2015.
* Rust target `*-pc-windows-gnu`: MinGW

```
$ cargo build --features build
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
