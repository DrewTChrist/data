use crate::device::ExtraPeripherals;
use crate::device::Sensor;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use tsl2591;

impl<D, I2C, I2CERR> Sensor<f32, D> for tsl2591::Driver<I2C>
where
    I2C: WriteRead<Error = I2CERR> + Write<Error = I2CERR>,
    I2CERR: core::fmt::Debug,
    D: embedded_hal::blocking::delay::DelayMs<u8>,
{
    fn read(&mut self, peripherals: &mut ExtraPeripherals<D>) -> f32 {
        if let Some(ref mut delay) = peripherals.delay {
            let (ch_0, ch_1) = self.get_channel_data(delay).unwrap();
            self.calculate_lux(ch_0, ch_1).unwrap()
        } else {
            0.0
        }
    }
}
