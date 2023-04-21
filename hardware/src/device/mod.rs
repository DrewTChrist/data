use core::marker::PhantomData;

/// Sensor trait to implement methods
/// that sensors need to have
pub trait Sensor<T> {
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
    fn read(&self) -> T;
}

/// Device is an abstraction over 
/// different hardware sensors and their
/// drivers
#[derive(Debug)]
pub struct Device<'a, S, T>
where
    S: Sensor<T>,
{
    /// The name of the device
    name: &'a str,
    /// The device driver that will
    /// read/write data
    driver: S,
    phantom: PhantomData<T>,
}

impl<'a, S, T> Device<'a, S, T>
where
    S: Sensor<T>,
{
    /// Creates a new Device
    pub fn new(name: &'a str, driver: S) -> Self {
        Self {
            name,
            driver,
            phantom: PhantomData,
        }
    }

    /// Gets the name of the device
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// Reads data from the device
    pub fn read(&self) -> T {
        self.driver.read()
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
