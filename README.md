# ut181a

[![Documentation](https://docs.rs/ut181a/badge.svg)](https://docs.rs/ut181a)

Uni-T UT181A digital multimeter (DMM) remote control library.
It supports USB connection only.

# Usage

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
