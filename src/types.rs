/// Marker trait for FS3000 device types (1005 vs 1015).
pub trait DeviceType: sealed::Sealed {
    /// A series of datapoints to translate from the raw measurement (12-bit integer) to meters per second (f32).
    fn datapoints() -> &'static [(u16, f32)];
}

/// A marker trait to indicate that we're communicating with a 1005 variant.
pub struct FS3000_1005;
/// A marker trait to indicate that we're communicating with a 1015 variant.
pub struct FS3000_1015;

impl DeviceType for FS3000_1005 {
    fn datapoints() -> &'static [(u16, f32)] {
        &[
            (409, 0.0),
            (915, 1.07),
            (1522, 2.01),
            (2066, 3.00),
            (2523, 3.97),
            (2908, 4.96),
            (3256, 5.98),
            (3572, 6.99),
            (3686, 7.23),
        ]
    }
}

impl DeviceType for FS3000_1015 {
    fn datapoints() -> &'static [(u16, f32)] {
        &[
            (409, 0.0),
            (1203, 2.0),
            (1597, 3.0),
            (1908, 4.0),
            (2187, 5.0),
            (2400, 6.0),
            (2629, 7.0),
            (2801, 8.0),
            (3006, 9.0),
            (3178, 10.0),
            (3309, 11.0),
            (3563, 13.0),
            (3686, 15.0),
        ]
    }
}

/// A marker trait to indicate whether the client is blocking or async.
pub trait ClientType: sealed::Sealed {}

/// A marker trait to indicate that the client is blocking.
pub struct Blocking;
/// A marker trait to indicate that the client is async.
pub struct Async;

impl ClientType for Blocking {}
impl ClientType for Async {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::FS3000_1005 {}
    impl Sealed for super::FS3000_1015 {}

    impl Sealed for super::Blocking {}
    impl Sealed for super::Async {}
}
