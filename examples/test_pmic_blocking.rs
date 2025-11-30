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

    info!("=== High-Level API Examples ===");

    // Enable ADC channels for battery and system monitoring (high-level API)
    axp.set_adc_channel_enable(AdcChannel::BatteryVoltage, true)?;
    axp.set_adc_channel_enable(AdcChannel::VbusVoltage, true)?;
    axp.set_adc_channel_enable(AdcChannel::VsysVoltage, true)?;
    axp.set_adc_channel_enable(AdcChannel::DieTemperature, true)?;

    // Configure DCDCs (high-level API)
    axp.set_dcdc_enable(DcId::Dcdc1, true)?;
    axp.set_dcdc_voltage(DcId::Dcdc1, 3300)?;

    // Configure LDOs (high-level API)
    axp.set_ldo_enable(LdoId::Aldo1, true)?;
    axp.set_ldo_voltage_mv(LdoId::Aldo1, 3300)?;

    // Read and display ADC values (high-level API)
    info!("Battery voltage: {} mV", axp.get_battery_voltage_mv()?);
    info!("VBUS voltage: {} mV", axp.get_vbus_voltage_mv()?);
    info!("VSYS voltage: {} mV", axp.get_vsys_voltage_mv()?);
    info!("Die temperature: {} C", axp.get_die_temperature_c()?);

    info!("=== Low-Level API Examples ===");

    // Read chip ID using low-level API
    let chip_id = axp.ll.chip_id().read()?;
    info!("Chip ID high: {}, version: {}, low: {}",
          chip_id.chip_id_high(), chip_id.chip_version() as u8, chip_id.chip_id_low());

    // Read battery percentage (SoC) using low-level API
    let soc = axp.ll.battery_percentage().read()?;
    info!("Battery SoC: {}%", soc.percentage());

    // Read power status using low-level API
    let status = axp.ll.power_status().read()?;
    info!("Battery present: {}", status.battery_present());
    info!("VBUS good: {}", status.vbus_good());

    // Configure battery detection using low-level API
    axp.ll.battery_detection_control().write(|w| {
        w.set_bat_det_en(true);
    })?;
    info!("Battery detection enabled via LL API");

    // Configure common config using low-level API
    axp.ll.common_config().write(|w| {
        w.set_discharge_off_enable(true);
        w.set_pwrok_restart_enable(true);
    })?;
    info!("PMU common config set via LL API");

    // Modify ADC channel enable (add one more channel) using low-level API
    // Note: modify() uses read-modify-write to preserve other fields
    axp.ll.adc_channel_enable_0().modify(|w| {
        w.set_gpadc_ch_en(true);  // Enable GPADC without affecting other channels
    })?;
    info!("GPADC channel enabled via LL API modify");

    Ok(())
}
