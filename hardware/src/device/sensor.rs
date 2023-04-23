use crate::device::Sensor;
use rand::Rng;

pub struct MockSensor;

impl MockSensor {
    pub fn new() -> Self {
        Self
    }
}

impl Sensor<f32> for MockSensor {
    fn read(&self) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }
}
