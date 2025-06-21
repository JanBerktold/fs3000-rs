/// The I2C address of the FS3000.
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
