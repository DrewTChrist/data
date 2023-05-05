use core::marker::PhantomData;
pub(crate) mod mock;
pub mod sensor;

/// Sensor trait to implement methods
/// that sensors need to have
pub trait Sensor<T, D>
where
    D: embedded_hal::blocking::delay::DelayMs<u8>,
{
    /// The read method should generalize the
    /// way sensors get read
    ///
    /// This method should read from the sensor
    /// and return the data value. If the sensor
    /// is capable of producing multiple values,
    /// return a struct:
    /// ```
    /// struct SensorData {
    ///     temp: f32,
    ///     air_qual: f32,
    ///     light: f32
    /// }
    /// ```
    fn read(&mut self, peripherals: &mut ExtraPeripherals<D>) -> T;
}

#[derive(Debug)]
pub struct ExtraPeripherals<D>
where
    D: embedded_hal::blocking::delay::DelayMs<u8>,
{
    pub delay: Option<D>,
}

/// Device is an abstraction over
/// different hardware sensors and their
/// drivers
#[derive(Debug)]
pub struct Device<'a, S, T, D>
where
    S: Sensor<T, D>,
    D: embedded_hal::blocking::delay::DelayMs<u8>,
{
    /// The name of the device
    name: &'a str,
    /// The device driver that will
    /// read/write data
    driver: S,
    peripherals: ExtraPeripherals<D>,
    phantom: PhantomData<T>,
}

impl<'a, S, T, D> Device<'a, S, T, D>
where
    S: Sensor<T, D>,
    D: embedded_hal::blocking::delay::DelayMs<u8>,
{
    /// Creates a new Device
    pub fn new(name: &'a str, driver: S) -> Self {
        Self {
            name,
            driver,
            peripherals: ExtraPeripherals { delay: None },
            phantom: PhantomData,
        }
    }

    /// Gets the name of the device
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// Reads data from the device
    pub fn read(&mut self) -> T {
        self.driver.read(&mut self.peripherals)
    }
}

/// The new_device macro creates a new
/// device given a device driver that
/// implements the Sensor trait
#[macro_export]
macro_rules! new_device {
    ($st:ident) => {
        Device::new(stringify!($st), <$st>::new())
    };
}
