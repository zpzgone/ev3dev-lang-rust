use crate::sensors::{Sensor, SensorPort};
use crate::{Attribute, Device, Driver, Ev3Result};

/// Angle
pub const MODE_GYRO_ANG: &str = "GYRO-ANG";

/// Rotational Speed
pub const MODE_GYRO_RATE: &str = "GYRO-RATE";

/// Raw sensor value ???
pub const MODE_GYRO_FAS: &str = "GYRO-FAS";

/// Angle and Rotational Speed
pub const MODE_GYRO_G_AND_A: &str = "GYRO-G&A";

/// Calibration ???
pub const MODE_GYRO_CAL: &str = "GYRO-CAL";

#[derive(Debug, Clone, Device)]
pub struct GyroSensor {
    driver: Driver,
}

impl Sensor for GyroSensor {}

impl GyroSensor {
    /// Try to get a `GyroSensor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: SensorPort) -> Ev3Result<GyroSensor> {
        let name = Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-gyro")?;

        Ok(GyroSensor {
            driver: Driver::new("lego-sensor", &name),
        })
    }

    /// Try to find a `GyroSensor`. Only returns a sensor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Ev3Result<GyroSensor> {
        let name = Driver::find_name_by_driver("lego-sensor", "lego-ev3-gyro")?;

        Ok(GyroSensor {
            driver: Driver::new("lego-sensor", &name),
        })
    }

    pub fn set_mode_col_ang(&self) -> Ev3Result<()> {
        self.set_mode(MODE_GYRO_ANG)
    }

    pub fn set_mode_col_rate(&self) -> Ev3Result<()> {
        self.set_mode(MODE_GYRO_RATE)
    }

    pub fn set_mode_col_fas(&self) -> Ev3Result<()> {
        self.set_mode(MODE_GYRO_FAS)
    }

    pub fn set_mode_gyro_g_and_a(&self) -> Ev3Result<()> {
        self.set_mode(MODE_GYRO_G_AND_A)
    }

    pub fn set_mode_gyro_cal(&self) -> Ev3Result<()> {
        self.set_mode(MODE_GYRO_CAL)
    }
}
