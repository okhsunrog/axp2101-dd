#![cfg_attr(not(any(test, feature = "std")), no_std)]
//! # AXP2101 Power Management Chip Interface
//!
//! This crate provides a bisync-based driver for the AXP2101 power management IC,
//! built upon the `device-driver` crate for robust, declarative register
//! definitions via a YAML manifest. It supports both asynchronous (`async`)
//! and blocking operation through a unified API, using the [`bisync`](https://docs.rs/bisync) crate
//! for seamless compatibility with both `embedded-hal` and `embedded-hal-async` traits.
//!
//! ## Features
//!
//! *   **Declarative Register Map:** Full device configuration defined in `device.yaml`.
//! *   **Unified Async/Blocking Support:** Write your code once and use it in both async and blocking contexts via bisync.
//! *   **Type-Safe API:** High-level functions for common operations (e.g., setting voltages)
//!     and a generated low-level API (`ll`) for direct register access.
//! *   **Comprehensive Register Coverage:** Aims to support the full feature set of the AXP2101.
//! *   **`defmt` and `log` Integration:** Optional support for logging and debugging.
//!
//! ## Getting Started
//!
//! To use the driver, instantiate `Axp2101` (blocking) or `Axp2101Async` (async) with your I2C bus implementation:
//!
//! ```rust,no_run
//! # use embedded_hal::i2c::I2c;
//! # use axp2101_dd::{Axp2101, DcId};
//! let i2c_bus = todo!();
//! let mut axp = Axp2101::new(i2c_bus);
//!
//! axp.set_dcdc_voltage(DcId::Dcdc1, 3300)?;
//! # Ok(())
//! ```
//!
//! For async environments, use `Axp2101Async` (re-exported from the `asynchronous` module):
//!
//! ```rust,no_run
//! # use embedded_hal_async::i2c::I2c;
//! # use axp2101_dd::{Axp2101Async, DcId};
//! let i2c_bus = todo!();
//! let mut axp = Axp2101Async::new(i2c_bus);
//!
//! axp.set_dcdc_voltage(DcId::Dcdc1, 3300).await?;
//! # Ok(())
//! ```
//!
//! For a detailed register map, please refer to the `device.yaml` file in the
//! [repository](https://github.com/okhsunrog/axp2101-dd).
//!
//! ## Supported Devices
//!
//! The AXP2101 is found in various embedded devices and development boards.
//!
//! ## Warning!
//!
//! ***Caution!*** This chip controls power to the microcontroller and other critical
//! components. Incorrect configuration can potentially damage or brick your device.
//! Proceed with care and always consult the AXP2101 datasheet.
//!

#[macro_use]
pub(crate) mod fmt;
// mod adc_helpers;  // TODO: Re-enable when ADC functionality is implemented

use thiserror::Error;

device_driver::create_device!(device_name: AxpLowLevel, manifest: "device.yaml");
pub const AXP2101_I2C_ADDRESS: u8 = 0x34;

#[derive(Debug, Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AxpError<I2cErr> {
    #[error("I2C error")]
    I2c(I2cErr),
    #[error("Invalid voltage: {0}mV for setting")]
    InvalidVoltage(u16),
    #[error("Invalid current: {0}mA for setting")]
    InvalidCurrent(u16),
    #[error("Feature or specific mode not supported/implemented: {0}")]
    NotSupported(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcId {
    Dcdc1,
    Dcdc2,
    Dcdc3,
    Dcdc4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LdoId {
    Aldo1,
    Aldo2,
    Aldo3,
    Aldo4,
    Bldo1,
    Bldo2,
    Dldo1,
    Dldo2,
    Cpusldo,
}

pub struct AxpInterface<I2CBus> {
    i2c_bus: I2CBus,
}

impl<I2CBus> AxpInterface<I2CBus> {
    pub fn new(i2c_bus: I2CBus) -> Self {
        Self { i2c_bus }
    }
}

#[path = "."]
mod asynchronous {
    use bisync::asynchronous::*;
    use device_driver::AsyncRegisterInterface as RegisterInterface;
    use embedded_hal_async::i2c::I2c;
    mod driver;
    pub use driver::*;
}
pub use asynchronous::Axp2101 as Axp2101Async;

#[path = "."]
mod blocking {
    use bisync::synchronous::*;
    use device_driver::RegisterInterface;
    use embedded_hal::i2c::I2c;
    #[allow(clippy::duplicate_mod)]
    mod driver;
    pub use driver::*;
}
pub use blocking::Axp2101;
