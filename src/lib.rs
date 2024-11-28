//! A platform-agnostic, embedded-hal driver for XXX.
#![no_std]

mod types;
pub use types::{DeviceType, FS3000_1005, FS3000_1015};

pub mod prelude {
    pub use crate::types::{DeviceType, FS3000_1005, FS3000_1015};
    pub use crate::{DeviceAddr, FS3000};
}

/// The I2C address of the FS3000.
#[derive(Copy, Clone, Debug, Default)]
pub enum DeviceAddr {
    /// The default address, 0x28.
    #[default]
    Default,
    /// Any other I2C address, e.g. when using a mux.
    Custom(u8),
}

impl From<DeviceAddr> for u8 {
    fn from(addr: DeviceAddr) -> u8 {
        match addr {
            DeviceAddr::Default => 0x28,
            DeviceAddr::Custom(addr) => addr,
        }
    }
}

pub struct FS3000<I2C, State: DeviceType> {
    address: DeviceAddr,
    i2c: I2C,
    _state: core::marker::PhantomData<State>,
}

impl<I2C, State: DeviceType> FS3000<I2C, State>
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

    pub fn connected(&mut self) -> Result<(), I2C::Error> {
        self.i2c.transaction(self.address.into(), &mut [])
    }

    pub fn read_meters_per_second(&mut self) -> Result<f32, I2C::Error> {
        let mut packet = Packet([0; 5]);
        self.i2c.read(self.address.into(), &mut packet.0)?;

        todo!()
    }
}

struct Packet([u8; 5]);
