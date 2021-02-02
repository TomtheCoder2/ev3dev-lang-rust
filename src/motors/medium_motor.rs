use super::MotorPort;
use crate::{Attribute, Device, Driver, Ev3Error, Ev3Result};
#[cfg(target_os = "linux")]
use crate::wait;
use std::time::Duration;

/// EV3 medium servo motor
#[derive(Debug, Clone, Device)]
pub struct MediumMotor {
    driver: Driver,
}

impl MediumMotor {

    fn new(driver: Driver) -> Self {
        Self {
            driver,
        }
    }

    findable!(
        "tacho-motor",
        "lego-ev3-m-motor",
        MotorPort,
        "MediumMotor",
        "out"
    );
    tacho_motor!();
}
