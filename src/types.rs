/// Marker trait for rocket states. Necessary level of indirection to avoid leaking private traits in public API.
pub trait DeviceType: sealed::Sealed {}

pub struct FS3000_1005;
pub struct FS3000_1015;

impl DeviceType for FS3000_1005 {}
impl DeviceType for FS3000_1015 {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::FS3000_1005 {}
    impl Sealed for super::FS3000_1015 {}
}
