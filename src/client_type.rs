/// Marker trait to support both blocking and async with the same client.
pub trait ClientType: sealed::Sealed {}

pub struct Blocking;
pub struct Async;

impl ClientType for Blocking {}
impl ClientType for Async {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::Blocking {}
    impl Sealed for super::Async {}
}
