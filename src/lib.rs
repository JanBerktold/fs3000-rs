//! A platform-agnostic, embedded-hal driver for FS3000 airflow sensors, either directly or via a [Sparkfun breakout board](https://www.sparkfun.com/products/18768).
#![no_std]
#![deny(missing_docs)]

mod address;
pub use address::DeviceAddr;
mod protocol;
mod types;
pub use types::{Async, Blocking, ClientType};

use protocol::Packet;
pub use types::{DeviceType, FS3000_1005, FS3000_1015};

/// Public module with all helpful types.
pub mod prelude {
    pub use crate::types::{FS3000_1005, FS3000_1015};
    pub use crate::{Async, Blocking, DeviceAddr, FS3000};
}

/// Any error that can occur when using this library.
#[derive(Debug, thiserror::Error)]
pub enum Error<I2CError> {
    /// A packet was received from the FS3000 but it's checksum was invalid.
    /// This typically indicates a faulty link or device.
    #[error("Checksum validation failed")]
    ChecksumFailed,

    /// Any I2C error that occurs when communicating with the device.
    #[error("I2C Error: {0:?}")]
    I2C(I2CError),
}

/// A client for a FS3000 device via I2C.
///
/// When creating this client, the consumer has two make decisions:
///     - Is the connected device a FS3000-1005 or FS3000-1015? The latter can measure larger air velocitys.
///     - Is the consuming code blocking or async?
///
/// Both of these decisions are documented using marker traits.
///
/// # Blocking Example
///
/// ```no_run
/// # #[derive(Default)]
/// # struct BlockingBus {};
/// #
/// # impl embedded_hal::i2c::ErrorType for BlockingBus {
/// #    type Error = core::convert::Infallible;
/// # }
/// #
/// # impl embedded_hal::i2c::I2c for BlockingBus {
/// #   fn transaction(&mut self, address: embedded_hal::i2c::SevenBitAddress, operations: &mut [embedded_hal::i2c::Operation<'_>],) -> Result<(), Self::Error> {
/// #     unimplemented!("hidden example code");
/// #   }
/// # }
/// use fs3000_rs::prelude::*;
///
/// // The [`BlockingBus`] is a fake `embedded_hal::i2c::I2c` for this example.
/// // In practice, you would create this [`embedded_hal::i2c::I2c`] via your platform hal.
/// let blocking_bus = BlockingBus::default();
///
/// // Assumes the FS3000-1015 (wider measurement range), substitute FS3000_
/// let mut client = FS3000::<FS3000_1015, Blocking, _>::new(DeviceAddr::default(), blocking_bus);
///
/// let mps = client.read_meters_per_second()?;
/// println!("We're going {mps} meters/second!");
///
/// # Ok::<(), fs3000_rs::Error<core::convert::Infallible>>(())
/// ```
///
/// # Async Example
///
/// ```no_run
/// # #[derive(Default)]
/// # struct AsyncBus {};
/// #
/// # impl embedded_hal::i2c::ErrorType for AsyncBus {
/// #    type Error = core::convert::Infallible;
/// # }
/// #
/// # impl embedded_hal_async::i2c::I2c for AsyncBus {
/// #   async fn transaction(&mut self, address: embedded_hal::i2c::SevenBitAddress, operations: &mut [embedded_hal::i2c::Operation<'_>],) -> Result<(), Self::Error> {
/// #     unimplemented!("hidden example code");
/// #   }
/// # }
/// use fs3000_rs::prelude::*;
///
/// // The [`BlockingBus`] is a fake `embedded_hal::i2c::I2c` for this example.
/// // In practice, you would create this [`embedded_hal::i2c::I2c`] via your platform hal.
/// let async_bus = AsyncBus::default();
///
/// # tokio_test::block_on(async {
/// let mut client = FS3000::<FS3000_1015, Async, _>::new(DeviceAddr::default(), async_bus);
///
/// let mps = client.read_meters_per_second().await.unwrap();
/// println!("We're going {mps} meters/second!");
/// # });
/// ```
pub struct FS3000<Device: DeviceType, Client: ClientType, I2C> {
    address: DeviceAddr,
    i2c: I2C,
    _client: core::marker::PhantomData<Client>,
    _state: core::marker::PhantomData<Device>,
}

impl<Device: DeviceType, I2C> FS3000<Device, Blocking, I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    /// Create a new FS3000 instance.
    pub fn new(address: DeviceAddr, i2c: I2C) -> Self {
        Self {
            i2c,
            address,
            _client: core::marker::PhantomData,
            _state: core::marker::PhantomData,
        }
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

impl<Device: DeviceType, I2C> FS3000<Device, Async, I2C>
where
    I2C: embedded_hal_async::i2c::I2c,
{
    /// Create a new FS3000 instance.
    pub fn new(address: DeviceAddr, i2c: I2C) -> Self {
        Self {
            i2c,
            address,
            _client: core::marker::PhantomData,
            _state: core::marker::PhantomData,
        }
    }

    /// Fetch a single, meters-per-second airflow measurement from the device.
    pub async fn read_meters_per_second(&mut self) -> Result<f32, Error<I2C::Error>> {
        let measurement = self.read_raw().await?;
        Ok(protocol::raw_to_meters_per_second::<Device>(measurement))
    }

    /// Fetch a single, raw measurement from the device.
    ///
    /// The measurement must be translated to a real unit for usage, consult
    /// the datasheet for details. Otherwise, use [`FS3000::read_meters_per_second`] to have
    /// this conversion be handled for you.
    pub async fn read_raw(&mut self) -> Result<u16, Error<I2C::Error>> {
        let mut packet = Packet([0; 5]);
        self.i2c
            .read(self.address.into(), &mut packet.0)
            .await
            .map_err(Error::<I2C::Error>::I2C)?;

        if !packet.valid() {
            return Err(Error::ChecksumFailed);
        }

        Ok(packet.measurement())
    }
}
