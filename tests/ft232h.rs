use fs3000_rs::prelude::*;
use fs3000_rs::FS3000;
use ftdi_embedded_hal::{
    libftd2xx::{Ft232h, Ftdi},
    FtHal, I2c,
};

#[test]
fn test_status() -> anyhow::Result<()> {
    let i2c = connect_i2c()?;

    let mut fs3000 = FS3000::<_, FS3000_1015>::new(DeviceAddr::Default, i2c);

    println!("Gathered");

    Ok(())
}

fn connect_i2c() -> anyhow::Result<I2c<Ft232h>> {
    let device = Ftdi::new()?;
    let device: Ft232h = device.try_into()?;

    let hal = FtHal::init_freq(device, 50_000)?;
    let i2c = hal.i2c()?;

    Ok(i2c)
}
