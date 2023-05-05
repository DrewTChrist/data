use crate::device::{ExtraPeripherals, Sensor};

#[derive(Debug)]
pub struct MockDelay;

impl embedded_hal::blocking::delay::DelayMs<u8> for MockDelay {
    fn delay_ms(&mut self, _: u8) {}
}

pub struct MockSensor(usize, [f32; 4]);

impl MockSensor {
    pub fn new() -> Self {
        Self(0, [1.0, 2.0, 3.0, 4.0])
    }
}

impl Iterator for MockSensor {
    type Item = f32;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.0 {
            x if x == 4 => {
                self.0 = 0;
                Some(self.1[self.0])
            }
            _ => {
                let v = Some(self.1[self.0]);
                self.0 += 1;
                v
            }
        }
    }
}

impl<D> Sensor<f32, D> for MockSensor
where
    D: embedded_hal::blocking::delay::DelayMs<u8>,
{
    fn read(&mut self, _peripherals: &mut ExtraPeripherals<D>) -> f32 {
        self.next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::ExtraPeripherals;
    use super::MockDelay;
    use super::MockSensor;
    use super::Sensor;

    #[test]
    fn next() {
        let mut sensor = MockSensor::new();
        let mut peripherals: ExtraPeripherals<MockDelay> = ExtraPeripherals { delay: None };
        assert_eq!(sensor.read(&mut peripherals), 1.0);
        assert_eq!(sensor.read(&mut peripherals), 2.0);
        assert_eq!(sensor.read(&mut peripherals), 3.0);
        assert_eq!(sensor.read(&mut peripherals), 4.0);
        assert_eq!(sensor.read(&mut peripherals), 1.0);
    }
}
