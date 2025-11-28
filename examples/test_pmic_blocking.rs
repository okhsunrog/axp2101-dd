#![no_std]
#![no_main]

use axp2101_dd::{AdcChannel, Axp2101, AxpError, DcId, LdoId};
use defmt::info;
use esp_hal::{
    Blocking,
    delay::Delay,
    i2c::master::{Config as I2cConfig, Error as I2cError, I2c},
    time::Rate,
};
use panic_rtt_target as _;
use rtt_target::rtt_init_defmt;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    rtt_init_defmt!();
    info!("Init!");

    let p = esp_hal::init(esp_hal::Config::default());
    let config: I2cConfig = I2cConfig::default().with_frequency(Rate::from_khz(400));
    let i2c = I2c::new(p.I2C0, config)
        .unwrap()
        .with_sda(p.GPIO6)
        .with_scl(p.GPIO7);

    init_pmic(i2c).unwrap();
    let delay = Delay::new();

    loop {
        info!("Hello world!");
        delay.delay_millis(250);
    }
}

#[rustfmt::skip]
fn init_pmic(i2c: I2c<'_, Blocking>) -> Result<(), AxpError<I2cError>> {
    let mut axp = Axp2101::new(i2c);

    // Enable ADC channels for battery and system monitoring
    axp.set_adc_channel_enable(AdcChannel::BatteryVoltage, true)?;
    axp.set_adc_channel_enable(AdcChannel::VbusVoltage, true)?;
    axp.set_adc_channel_enable(AdcChannel::VsysVoltage, true)?;
    axp.set_adc_channel_enable(AdcChannel::DieTemperature, true)?;

    // Configure DCDCs
    axp.set_dcdc_enable(DcId::Dcdc1, true)?;
    axp.set_dcdc_voltage(DcId::Dcdc1, 3300)?;

    // Configure LDOs
    axp.set_ldo_enable(LdoId::Aldo1, true)?;
    axp.set_ldo_voltage_mv(LdoId::Aldo1, 3300)?;

    // Read and display ADC values
    info!("Battery voltage: {} mV", axp.get_battery_voltage_mv()?);
    info!("VBUS voltage: {} mV", axp.get_vbus_voltage_mv()?);
    info!("VSYS voltage: {} mV", axp.get_vsys_voltage_mv()?);
    info!("Die temperature: {} C", axp.get_die_temperature_c()?);

    Ok(())
}
