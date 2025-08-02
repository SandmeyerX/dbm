mod bluetooth;

use bluetooth::{Device, BtDevice};

fn main() {
    let device = Device::new();
    dbg!(device.disconnect());
}
