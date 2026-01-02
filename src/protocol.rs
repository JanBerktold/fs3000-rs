use crate::DeviceType;

pub(crate) struct Packet(pub [u8; 5]);

impl Packet {
    pub fn measurement(&self) -> u16 {
        let mut data_high: u16 = self.0[1] as u16;
        let data_low: u16 = self.0[2] as u16;

        // The flow data is a 12-bit integer.
        // Only the least significant four bits in the high byte are valid.
        // clear out (mask out) the unnecessary bits
        data_high &= 0b00001111;

        let mut airflow: u16 = 0;
        airflow |= data_low;
        airflow |= data_high << 8;

        airflow
    }

    pub fn valid(&self) -> bool {
        // Sum all bytes (except the crc-byte at the beginning), while allowing overflows.
        let sum: u8 = self.0.iter().skip(1).fold(0u8, |a, b| a.wrapping_add(*b));

        let crcbyte = self.0[0];
        let overall = sum.wrapping_add(crcbyte);

        overall == 0x00
    }
}

pub(crate) fn raw_to_meters_per_second<D: DeviceType>(measurement: u16) -> f32 {
    let translation_points = D::datapoints();

    // Get the lowest datapoint for which our measurement is equal or higher.
    let Some(index) = translation_points
        .iter()
        .enumerate()
        .rev()
        .find(|(_, (raw, _))| measurement >= *raw)
        .map(|(index, _)| index)
    else {
        // If we're smaller than the first datapoint, then we can short-circuit and return 0.0.
        return 0.0;
    };

    // Get the two adjacent datapoints -- one lower, one higher.
    let (lower, higher) = match (
        translation_points.get(index),
        translation_points.get(index + 1),
    ) {
        (Some(lower), Some(higher)) => (lower, higher),
        // If our lower one is the highest datapoint already, then we just return that bound.
        (Some(lower), None) => return lower.1,
        _ => unreachable!("lower index must always exist"),
    };

    // Assume that the curve between our two datapoints is linear, and use that to
    // interpolate our reading.
    let window_size = higher.0 - lower.0;
    let difference_to_bottom = measurement - lower.0;
    let window_percentage = (difference_to_bottom as f32) / window_size as f32;

    let window_size_meters_per_second = higher.1 - lower.1;

    lower.1 + (window_size_meters_per_second * window_percentage)
}

#[cfg(test)]
mod tests {
    use crate::FS3000_1005;

    use super::*;

    // TODO: check for measurement

    #[test]
    fn test_checksum() {
        // A bogus, invalid packet.
        let packet = Packet([0x00, 0x01, 0x02, 0x03, 0x04]);
        assert!(!packet.valid());

        // Example taking from the datasheet on page 10.
        let packet = Packet([0xCC, 0x01, 0x99, 0x01, 0x99]);
        assert!(packet.valid());
    }

    #[test]
    fn test_raw_to_meters_per_second_1005() {
        macro_rules! assert_with_error {
            ($input:expr, $b:expr) => {
                let result = raw_to_meters_per_second::<FS3000_1005>($input);
                let diff = (result - $b).abs();
                assert!(
                    diff < 1e-3,
                    "Expected {} and got {}, which differs by {} (allowed error 1e-3) for input {}",
                    $b,
                    result,
                    diff,
                    $input
                );
            };
        }

        // Use known datapoints for FS3000_1005
        assert_with_error!(409, 0.0);
        assert_with_error!(915, 1.07);
        assert_with_error!(1522, 2.01);
        assert_with_error!(3686, 7.23);

        // Test interpolation between two points -- 50% between 915 and 1522.
        let mid = (915 + 1522) / 2;
        let expected = (1.07 + 2.01) / 2.0;
        assert_with_error!(mid, expected);
    }
}
