#![no_std]
#![no_main]

use axp2101_dd::{AdcChannel, Axp2101Async, AxpError, DcId, LdoId};
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{
    Async,
    i2c::master::{Config as I2cConfig, Error as I2cError, I2c},
    interrupt::software::SoftwareInterruptControl,
    time::Rate,
    timer::timg::TimerGroup,
};
use panic_rtt_target as _;
use rtt_target::rtt_init_defmt;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_spawner: Spawner) {
    rtt_init_defmt!();
    info!("Init!");

    let p = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(p.TIMG0);
    let sw_ints = SoftwareInterruptControl::new(p.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_ints.software_interrupt0);

    let config: I2cConfig = I2cConfig::default().with_frequency(Rate::from_khz(400));
    let i2c = I2c::new(p.I2C0, config)
        .unwrap()
        .with_sda(p.GPIO6)
        .with_scl(p.GPIO7)
        .into_async();

    init_pmic(i2c).await.unwrap();

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[rustfmt::skip]
async fn init_pmic(i2c: I2c<'_, Async>) -> Result<(), AxpError<I2cError>> {
    let mut axp = Axp2101Async::new(i2c);

    info!("=== High-Level API Examples ===");

    // Enable ADC channels for battery and system monitoring (high-level API)
    axp.set_adc_channel_enable(AdcChannel::BatteryVoltage, true).await?;
    axp.set_adc_channel_enable(AdcChannel::VbusVoltage, true).await?;
    axp.set_adc_channel_enable(AdcChannel::VsysVoltage, true).await?;
    axp.set_adc_channel_enable(AdcChannel::DieTemperature, true).await?;

    // Configure DCDCs (high-level API)
    axp.set_dcdc_enable(DcId::Dcdc1, true).await?;
    axp.set_dcdc_voltage(DcId::Dcdc1, 3300).await?;

    // Configure LDOs (high-level API)
    axp.set_ldo_enable(LdoId::Aldo1, true).await?;
    axp.set_ldo_voltage_mv(LdoId::Aldo1, 3300).await?;

    // Read and display ADC values (high-level API)
    info!("Battery voltage: {} mV", axp.get_battery_voltage_mv().await?);
    info!("VBUS voltage: {} mV", axp.get_vbus_voltage_mv().await?);
    info!("VSYS voltage: {} mV", axp.get_vsys_voltage_mv().await?);
    info!("Die temperature: {} C", axp.get_die_temperature_c().await?);

    info!("=== Low-Level API Examples ===");

    // Read chip ID using low-level API async
    let chip_id = axp.ll.chip_id().read_async().await?;
    info!("Chip ID high: {}, version: {}, low: {}",
          chip_id.chip_id_high(), chip_id.chip_version() as u8, chip_id.chip_id_low());

    // Read battery percentage (SoC) using low-level API
    let soc = axp.ll.battery_percentage().read_async().await?;
    info!("Battery SoC: {}%", soc.percentage());

    // Read power status using low-level API
    let status = axp.ll.power_status().read_async().await?;
    info!("Battery present: {}", status.battery_present());
    info!("VBUS good: {}", status.vbus_good());

    // Configure battery detection using low-level API
    axp.ll.battery_detection_control().write_async(|w| {
        w.set_bat_det_en(true);
    }).await?;
    info!("Battery detection enabled via LL API");

    // Configure common config using low-level API
    axp.ll.common_config().write_async(|w| {
        w.set_discharge_off_enable(true);
        w.set_pwrok_restart_enable(true);
    }).await?;
    info!("PMU common config set via LL API");

    // Modify ADC channel enable (add one more channel) using low-level API
    // Note: modify_async() uses read-modify-write to preserve other fields
    axp.ll.adc_channel_enable_0().modify_async(|w| {
        w.set_gpadc_ch_en(true);  // Enable GPADC without affecting other channels
    }).await?;
    info!("GPADC channel enabled via LL API modify");

    Ok(())
}
