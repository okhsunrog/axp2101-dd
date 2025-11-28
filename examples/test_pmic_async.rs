#![no_std]
#![no_main]

use axp2101_dd::{AdcChannel, Axp2101Async, AxpError, DcId, LdoId};
use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::Async;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{
    i2c::master::{Config as I2cConfig, Error as I2cError, I2c},
    time::Rate,
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    let config = esp_hal::Config::default();
    let p = esp_hal::init(config);

    let timer0 = TimerGroup::new(p.TIMG0);
    esp_hal_embassy::init(timer0.timer0);
    info!("Embassy initialized!");

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

    // Enable ADC channels for battery and system monitoring
    axp.set_adc_channel_enable(AdcChannel::BatteryVoltage, true).await?;
    axp.set_adc_channel_enable(AdcChannel::VbusVoltage, true).await?;
    axp.set_adc_channel_enable(AdcChannel::VsysVoltage, true).await?;
    axp.set_adc_channel_enable(AdcChannel::DieTemperature, true).await?;

    // Configure DCDCs
    axp.set_dcdc_enable(DcId::Dcdc1, true).await?;
    axp.set_dcdc_voltage(DcId::Dcdc1, 3300).await?;

    // Configure LDOs
    axp.set_ldo_enable(LdoId::Aldo1, true).await?;
    axp.set_ldo_voltage_mv(LdoId::Aldo1, 3300).await?;

    // Read and display ADC values
    info!("Battery voltage: {} mV", axp.get_battery_voltage_mv().await?);
    info!("VBUS voltage: {} mV", axp.get_vbus_voltage_mv().await?);
    info!("VSYS voltage: {} mV", axp.get_vsys_voltage_mv().await?);
    info!("Die temperature: {} C", axp.get_die_temperature_c().await?);

    Ok(())
}
