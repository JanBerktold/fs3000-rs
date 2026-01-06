# embedded_hal device driver for FS3000 air velocity sensors

![docs.rs](https://img.shields.io/docsrs/fs3000-rs)

This crate supports I2C communication to any [FS3000](https://www.renesas.com/en/document/dst/fs3000-datasheet?srsltid=AfmBOorwz1ZnOTXa22fXgrCYT0A0dsdPv8fkIC_GyIuNdDE7nEYVhDZ3) air velocity sensor such as the `FS3000-1015`. They are commonly available, for example as a [Sparkfun Qwiic Board](https://www.sparkfun.com/products/18768).

As this crate builds upon [embedded-hal](https://docs.rs/crate/embedded-hal/latest) and [embedded-hal-async](https://docs.rs/crate/embedded-hal-async/latest), it can be used in either blocking or [async](https://embassy.dev/book/index.html) code on all platforms supported by `embedded-hal`.

See the [FS3000 documentation](https://docs.rs/fs3000-rs/latest/fs3000_rs/struct.FS3000.html) for usage examples.