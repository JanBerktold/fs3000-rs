//! A platform-agnostic, embedded-hal driver for FS3000 airflow sensors, either directly or via a [Sparkfun breakout board](https://www.sparkfun.com/products/18768).
#![no_std]
#[deny(missing_docs)]
mod address;
pub use address::DeviceAddr;
mod protocol;
mod types;

use protocol::Packet;
pub use types::{DeviceType, FS3000_1005, FS3000_1015};

/// Public module with all helpful types.
pub mod prelude {
    pub use crate::types::{FS3000_1005, FS3000_1015};
    pub use crate::{DeviceAddr, FS3000};
}

#[derive(Debug, thiserror::Error)]
pub enum Error<I2CError> {
    #[error("Checksum validation failed")]
    ChecksumFailed,

    #[error("I2C Error: {0:?}")]
    I2C(I2CError),
}

pub struct FS3000<Device: DeviceType, I2C> {
    address: DeviceAddr,
    i2c: I2C,
    _state: core::marker::PhantomData<Device>,
}

impl<Device: DeviceType, I2C> FS3000<Device, I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    /// Create a new FS3000 instance.
    pub fn new(address: DeviceAddr, i2c: I2C) -> Self {
        Self {
            i2c,
            address,
            _state: core::marker::PhantomData,
        }
    }

    /// Check whether the FS3000 device is reachable.
    pub fn connected(&mut self) -> Result<(), I2C::Error> {
        self.i2c.transaction(self.address.into(), &mut [])
    }

    /// Fetch a single, meters-per-second airflow measurement from the device.
    pub fn read_meters_per_second(&mut self) -> Result<f32, Error<I2C::Error>> {
        let measurement = self.read_raw()?;
        Ok(protocol::raw_to_meters_per_second::<Device>(measurement))
    }

    /// Fetch a single, raw measurement from the device.
    ///
    /// The measurement must be translated to a real unit for usage, consult
    /// the datasheet for details. Otherwise, use [`FS3000::read_meters_per_second`] to have
    /// this conversion be handled for you.
    pub fn read_raw(&mut self) -> Result<u16, Error<I2C::Error>> {
        let mut packet = Packet([0; 5]);
        self.i2c
            .read(self.address.into(), &mut packet.0)
            .map_err(Error::<I2C::Error>::I2C)?;

        if !packet.valid() {
            return Err(Error::ChecksumFailed);
        }

        Ok(packet.measurement())
    }
}
