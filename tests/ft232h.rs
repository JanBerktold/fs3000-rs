//! A suite of local integration tests that require a FT232H device plugged in, with a FS3000 connected to it's I2C bus.
//!
//! TODO: Add an async test suite -- need to find an async FT232H hal.
use std::cell::RefCell;

use embedded_hal_bus::i2c::RefCellDevice;
use fs3000_rs::prelude::*;
use fs3000_rs::FS3000;
use ftdi_embedded_hal::{
    libftd2xx::{Ft232h, Ftdi},
    FtHal, I2c,
};

#[test]
fn test_blocking() -> anyhow::Result<()> {
    let i2c = RefCell::new(connect_i2c()?);

    let mut fs3000 =
        FS3000::<FS3000_1015, Blocking, _>::new(DeviceAddr::Default, RefCellDevice::new(&i2c));

    let mps = fs3000.read_meters_per_second().expect("to read");
    println!("meter per second: {:?}", mps);

    Ok(())
}

fn connect_i2c() -> anyhow::Result<I2c<Ft232h>> {
    let device = Ftdi::new()?;
    let device: Ft232h = device.try_into()?;

    let hal = FtHal::init_freq(device, 24_000)?;
    let i2c = hal.i2c()?;

    Ok(i2c)
}
