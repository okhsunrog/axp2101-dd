use super::{I2c, RegisterInterface, bisync, only_async, only_sync};
use crate::adc_helpers::*;
use crate::{AXP2101_I2C_ADDRESS, AdcChannel, AxpError, AxpInterface, AxpLowLevel, DcId, LdoId};
use crate::{BatteryCurrentDirection, ChargeVoltageLimit, FastChargeCurrentLimit};

#[bisync]
impl<I2CBus, E> RegisterInterface for AxpInterface<I2CBus>
where
    I2CBus: I2c<Error = E>,
    E: core::fmt::Debug,
{
    type AddressType = u8;
    type Error = AxpError<E>;
    async fn read_register(
        &mut self,
        address: u8,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c_bus
            .write_read(AXP2101_I2C_ADDRESS, &[address], data)
            .await
            .map_err(AxpError::I2c)
    }
    async fn write_register(
        &mut self,
        address: u8,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut buffer = [0u8; 5];
        if (1 + data.len()) > buffer.len() {
            return Err(AxpError::NotSupported("Write data length exceeds buffer"));
        }
        buffer[0] = address;
        buffer[1..1 + data.len()].copy_from_slice(data);
        self.i2c_bus
            .write(AXP2101_I2C_ADDRESS, &buffer[..1 + data.len()])
            .await
            .map_err(AxpError::I2c)
    }
}

pub struct Axp2101<
    I2CImpl: RegisterInterface<AddressType = u8, Error = AxpError<I2CBusErr>>,
    I2CBusErr: core::fmt::Debug = <I2CImpl as RegisterInterface>::Error,
> {
    pub ll: AxpLowLevel<I2CImpl>,
    _marker: core::marker::PhantomData<I2CBusErr>,
}

impl<I2CBus, E> Axp2101<AxpInterface<I2CBus>, E>
where
    I2CBus: I2c<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(i2c: I2CBus) -> Self {
        Self {
            ll: AxpLowLevel::new(AxpInterface::new(i2c)),
            _marker: core::marker::PhantomData,
        }
    }
}

pub trait CurrentAxpDriverInterface<E>:
    RegisterInterface<AddressType = u8, Error = AxpError<E>>
{
}

impl<T, E> CurrentAxpDriverInterface<E> for T
where
    T: RegisterInterface<AddressType = u8, Error = AxpError<E>>,
    E: core::fmt::Debug,
{
}

include!("bisync_helpers.rs");

impl<I2CImpl, I2CBusErr> Axp2101<I2CImpl, I2CBusErr>
where
    I2CImpl: CurrentAxpDriverInterface<I2CBusErr>,
    I2CBusErr: core::fmt::Debug,
{
    #[bisync]
    pub async fn get_battery_voltage_mv(&mut self) -> Result<u16, AxpError<I2CBusErr>> {
        let mut op_high = self.ll.battery_voltage_adc_high();
        let high_byte = read_internal(&mut op_high).await?;
        let mut op_low = self.ll.battery_voltage_adc_low();
        let low_byte = read_internal(&mut op_low).await?;

        let adc_value = adc_14bit_combine(high_byte.value(), low_byte.value());
        Ok(battery_voltage_to_mv(adc_value))
    }

    #[bisync]
    pub async fn get_vbus_voltage_mv(&mut self) -> Result<u16, AxpError<I2CBusErr>> {
        let mut op_high = self.ll.vbus_voltage_adc_high();
        let high_byte = read_internal(&mut op_high).await?;
        let mut op_low = self.ll.vbus_voltage_adc_low();
        let low_byte = read_internal(&mut op_low).await?;

        let adc_value = adc_14bit_combine(high_byte.value(), low_byte.value());
        Ok(vbus_voltage_to_mv(adc_value))
    }

    #[bisync]
    pub async fn get_vsys_voltage_mv(&mut self) -> Result<u16, AxpError<I2CBusErr>> {
        let mut op_high = self.ll.vsys_voltage_adc_high();
        let high_byte = read_internal(&mut op_high).await?;
        let mut op_low = self.ll.vsys_voltage_adc_low();
        let low_byte = read_internal(&mut op_low).await?;

        let adc_value = adc_14bit_combine(high_byte.value(), low_byte.value());
        Ok(vsys_voltage_to_mv(adc_value))
    }

    #[bisync]
    pub async fn get_ts_pin_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let mut op_high = self.ll.ts_pin_adc_high();
        let high_byte = read_internal(&mut op_high).await?;
        let mut op_low = self.ll.ts_pin_adc_low();
        let low_byte = read_internal(&mut op_low).await?;

        let adc_value = adc_14bit_combine(high_byte.value(), low_byte.value());
        Ok(ts_pin_to_mv(adc_value))
    }

    #[bisync]
    pub async fn get_die_temperature_c(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let mut op_high = self.ll.internal_temperature_adc_high();
        let high_byte = read_internal(&mut op_high).await?;
        let mut op_low = self.ll.internal_temperature_adc_low();
        let low_byte = read_internal(&mut op_low).await?;

        let adc_value = adc_14bit_combine(high_byte.value(), low_byte.value());
        Ok(die_temp_to_celsius(adc_value))
    }

    #[bisync]
    pub async fn get_gpadc_value(&mut self) -> Result<u16, AxpError<I2CBusErr>> {
        let mut op_high = self.ll.gpadc_adc_high();
        let high_byte = read_internal(&mut op_high).await?;
        let mut op_low = self.ll.gpadc_adc_low();
        let low_byte = read_internal(&mut op_low).await?;

        Ok(adc_14bit_combine(high_byte.value(), low_byte.value()))
    }

    #[bisync]
    pub async fn set_dcdc_enable(
        &mut self,
        dc: DcId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.dcdc_config_0();
        match dc {
            DcId::Dcdc1 => modify_internal(&mut op, |r| r.set_dcdc1_enable(enable)).await,
            DcId::Dcdc2 => modify_internal(&mut op, |r| r.set_dcdc2_enable(enable)).await,
            DcId::Dcdc3 => modify_internal(&mut op, |r| r.set_dcdc3_enable(enable)).await,
            DcId::Dcdc4 => modify_internal(&mut op, |r| r.set_dcdc4_enable(enable)).await,
            DcId::Dcdc5 => modify_internal(&mut op, |r| r.set_dcdc5_enable(enable)).await,
        }
    }

    #[bisync]
    pub async fn set_dcdc_voltage(
        &mut self,
        dc: DcId,
        voltage_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let voltage_val = match dc {
            // DCDC1: 1.5–3.4 V, 100 mV/step
            DcId::Dcdc1 => {
                if !(1500..=3400).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                ((voltage_mv - 1500) / 100) as u8
            }
            // DCDC2/3: 0.5–1.2 V (10 mV), 1.22–1.54 V (20 mV); DCDC3 extra 1.6–3.4 V (100 mV)
            DcId::Dcdc2 | DcId::Dcdc3 => {
                if (500..=1200).contains(&voltage_mv) {
                    ((voltage_mv - 500) / 10) as u8
                } else if (1220..=1540).contains(&voltage_mv) {
                    (71 + (voltage_mv - 1220) / 20) as u8
                } else if dc == DcId::Dcdc3 && (1600..=3400).contains(&voltage_mv) {
                    (88 + (voltage_mv - 1600) / 100) as u8
                } else {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
            }
            // DCDC4: 0.5–1.2 V (10 mV), 1.22–1.84 V (20 mV)
            DcId::Dcdc4 => {
                if (500..=1200).contains(&voltage_mv) {
                    ((voltage_mv - 500) / 10) as u8
                } else if (1220..=1840).contains(&voltage_mv) {
                    (71 + (voltage_mv - 1220) / 20) as u8
                } else {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
            }
            // DCDC5: 1.4–3.7 V, 100 mV/step
            DcId::Dcdc5 => {
                if !(1400..=3700).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                ((voltage_mv - 1400) / 100) as u8
            }
        };

        match dc {
            DcId::Dcdc1 => {
                let mut op = self.ll.dcdc_1_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(voltage_val)).await
            }
            DcId::Dcdc2 => {
                let mut op = self.ll.dcdc_2_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(voltage_val)).await
            }
            DcId::Dcdc3 => {
                let mut op = self.ll.dcdc_3_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(voltage_val)).await
            }
            DcId::Dcdc4 => {
                let mut op = self.ll.dcdc_4_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(voltage_val)).await
            }
            DcId::Dcdc5 => {
                let mut op = self.ll.dcdc_5_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(voltage_val)).await
            }
        }
    }

    #[bisync]
    pub async fn set_ldo_enable(
        &mut self,
        ldo: LdoId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        match ldo {
            LdoId::Dldo2 => {
                let mut op = self.ll.ldo_enable_config_1();
                modify_internal(&mut op, |r| r.set_dldo2_enable(enable)).await
            }
            _ => {
                let mut op = self.ll.ldo_enable_config_0();
                modify_internal(&mut op, |r| match ldo {
                    LdoId::Aldo1 => r.set_aldo1_enable(enable),
                    LdoId::Aldo2 => r.set_aldo2_enable(enable),
                    LdoId::Aldo3 => r.set_aldo3_enable(enable),
                    LdoId::Aldo4 => r.set_aldo4_enable(enable),
                    LdoId::Bldo1 => r.set_bldo1_enable(enable),
                    LdoId::Bldo2 => r.set_bldo2_enable(enable),
                    LdoId::Dldo1 => r.set_dldo1_enable(enable),
                    LdoId::Cpusldo => r.set_cpusldo_enable(enable),
                    LdoId::Dldo2 => unreachable!(),
                })
                .await
            }
        }
    }

    #[bisync]
    pub async fn set_ldo_voltage_mv(
        &mut self,
        ldo: LdoId,
        voltage_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        match ldo {
            LdoId::Aldo1 => {
                if !(500..=3500).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 100) as u8;
                let mut op = self.ll.aldo_1_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(val)).await
            }
            LdoId::Aldo2 => {
                if !(500..=3500).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 100) as u8;
                let mut op = self.ll.aldo_2_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(val)).await
            }
            LdoId::Aldo3 => {
                if !(500..=3500).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 100) as u8;
                let mut op = self.ll.aldo_3_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(val)).await
            }
            LdoId::Aldo4 => {
                if !(500..=3500).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 100) as u8;
                let mut op = self.ll.aldo_4_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(val)).await
            }
            LdoId::Bldo1 => {
                if !(500..=3500).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 100) as u8;
                let mut op = self.ll.bldo_1_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(val)).await
            }
            LdoId::Bldo2 => {
                if !(500..=3500).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 100) as u8;
                let mut op = self.ll.bldo_2_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(val)).await
            }
            LdoId::Dldo1 => {
                if !(500..=3500).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 100) as u8;
                let mut op = self.ll.dldo_1_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(val)).await
            }
            LdoId::Dldo2 => {
                if !(500..=1400).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 50) as u8;
                let mut op = self.ll.dldo_2_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(val)).await
            }
            LdoId::Cpusldo => {
                if !(500..=1400).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 50) as u8;
                let mut op = self.ll.cpu_sldo_voltage_config();
                modify_internal(&mut op, |r| r.set_voltage_setting(val)).await
            }
        }
    }

    #[bisync]
    pub async fn set_battery_charge_enable(
        &mut self,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.module_enable();
        modify_internal(&mut op, |r| r.set_battery_charge_enable(enable)).await
    }

    #[bisync]
    pub async fn set_battery_charge_current(
        &mut self,
        current_setting: FastChargeCurrentLimit,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.fast_charge_current_config();
        modify_internal(&mut op, |r| r.set_fast_charge_current(current_setting)).await
    }

    #[bisync]
    pub async fn set_battery_charge_voltage(
        &mut self,
        voltage_setting: ChargeVoltageLimit,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.charge_voltage_config();
        modify_internal(&mut op, |r| r.set_charge_voltage(voltage_setting)).await
    }

    #[bisync]
    pub async fn get_chip_id(&mut self) -> Result<u8, AxpError<I2CBusErr>> {
        let mut op = self.ll.chip_id();
        let chip_data = read_internal(&mut op).await?;
        // Compose chip ID from high bits [7:6], version [5:4], and low bits [3:0]
        let chip_id = ((chip_data.chip_id_high() as u8) << 6)
            | ((chip_data.chip_version() as u8) << 4)
            | (chip_data.chip_id_low() as u8);
        Ok(chip_id)
    }

    #[bisync]
    pub async fn get_power_status(
        &mut self,
    ) -> Result<(bool, bool, bool, bool), AxpError<I2CBusErr>> {
        let mut op = self.ll.power_status();
        let status = read_internal(&mut op).await?;

        Ok((
            status.vbus_good(),
            status.batfet_state(),
            status.battery_present(),
            status.current_limit_active(),
        ))
    }

    #[bisync]
    pub async fn get_battery_status(&mut self) -> Result<(bool, bool), AxpError<I2CBusErr>> {
        // Get battery present from power status
        let mut op1 = self.ll.power_status();
        let power_status = read_internal(&mut op1).await?;

        // Get charging status from system status
        let mut op2 = self.ll.system_status();
        let system_status = read_internal(&mut op2).await?;

        Ok((
            power_status.battery_present(),
            // Check if currently charging (not standby, not discharging)
            matches!(
                system_status.battery_current_direction(),
                BatteryCurrentDirection::Charging
            ),
        ))
    }

    /// Enable or disable a specific ADC channel
    #[bisync]
    pub async fn set_adc_channel_enable(
        &mut self,
        channel: AdcChannel,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.adc_channel_enable_0();
        modify_internal(&mut op, |r| match channel {
            AdcChannel::BatteryVoltage => r.set_vbat_ch_en(enable),
            AdcChannel::TsPin => r.set_ts_ch_en(enable),
            AdcChannel::VbusVoltage => r.set_vbus_ch_en(enable),
            AdcChannel::VsysVoltage => r.set_vsys_ch_en(enable),
            AdcChannel::DieTemperature => r.set_tdie_ch_en(enable),
            AdcChannel::Gpadc => r.set_gpadc_ch_en(enable),
        })
        .await
    }

    /// Enable all ADC channels at once
    #[bisync]
    pub async fn enable_all_adc_channels(&mut self) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.adc_channel_enable_0();
        modify_internal(&mut op, |r| {
            r.set_vbat_ch_en(true);
            r.set_ts_ch_en(true);
            r.set_vbus_ch_en(true);
            r.set_vsys_ch_en(true);
            r.set_tdie_ch_en(true);
            r.set_gpadc_ch_en(true);
        })
        .await
    }

    /// Enable ADC channels using a bitmask (legacy API)
    /// Bits: [5]=GPADC, [4]=TDIE, [3]=VSYS, [2]=VBUS, [1]=TS, [0]=VBAT
    #[bisync]
    pub async fn enable_adc_channel(
        &mut self,
        channel_mask: u8,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.adc_channel_enable_0();
        modify_internal(&mut op, |r| {
            r.set_gpadc_ch_en(channel_mask & (1 << 5) != 0);
            r.set_tdie_ch_en(channel_mask & (1 << 4) != 0);
            r.set_vsys_ch_en(channel_mask & (1 << 3) != 0);
            r.set_vbus_ch_en(channel_mask & (1 << 2) != 0);
            r.set_ts_ch_en(channel_mask & (1 << 1) != 0);
            r.set_vbat_ch_en(channel_mask & (1 << 0) != 0);
        })
        .await
    }

    #[bisync]
    pub async fn enable_interrupts(
        &mut self,
        irq0_mask: u8,
        irq1_mask: u8,
    ) -> Result<(), AxpError<I2CBusErr>> {
        // Enable IRQ0 register interrupts (battery/gauge related)
        let mut op0 = self.ll.irq_enable_0();
        write_internal(&mut op0, |r| {
            r.set_soc_warning_level2_irq_enable(irq0_mask & 0x80 != 0);
            r.set_soc_warning_level1_irq_enable(irq0_mask & 0x40 != 0);
            r.set_gauge_watchdog_timeout_irq_enable(irq0_mask & 0x20 != 0);
            r.set_new_soc_irq_enable(irq0_mask & 0x10 != 0);
            r.set_battery_charge_over_temp_irq_enable(irq0_mask & 0x08 != 0);
            r.set_battery_charge_under_temp_irq_enable(irq0_mask & 0x04 != 0);
            r.set_battery_work_over_temp_irq_enable(irq0_mask & 0x02 != 0);
            r.set_battery_work_under_temp_irq_enable(irq0_mask & 0x01 != 0);
        })
        .await?;

        // Enable IRQ1 register interrupts (VBUS/power key related)
        let mut op1 = self.ll.irq_enable_1();
        write_internal(&mut op1, |r| {
            r.set_vbus_insert_irq_enable(irq1_mask & 0x80 != 0);
            r.set_vbus_remove_irq_enable(irq1_mask & 0x40 != 0);
            r.set_battery_insert_irq_enable(irq1_mask & 0x20 != 0);
            r.set_battery_remove_irq_enable(irq1_mask & 0x10 != 0);
            r.set_power_key_short_press_irq_enable(irq1_mask & 0x08 != 0);
            r.set_power_key_long_press_irq_enable(irq1_mask & 0x04 != 0);
            r.set_power_key_negative_edge_irq_enable(irq1_mask & 0x02 != 0);
            r.set_power_key_positive_edge_irq_enable(irq1_mask & 0x01 != 0);
        })
        .await
    }

    #[bisync]
    pub async fn enable_interrupts2(
        &mut self,
        irq2_mask: u8,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op2 = self.ll.irq_enable_2();
        write_internal(&mut op2, |r| {
            r.set_wdexp_irq_en(irq2_mask & 0x80 != 0);
            r.set_ldooc_irq_en(irq2_mask & 0x40 != 0);
            r.set_bocp_irq_en(irq2_mask & 0x20 != 0);
            r.set_chgdn_irq_en(irq2_mask & 0x10 != 0);
            r.set_chgst_irq_en(irq2_mask & 0x08 != 0);
            r.set_dot11_irq_en(irq2_mask & 0x04 != 0);
            r.set_chgte_irq_en(irq2_mask & 0x02 != 0);
            r.set_bovp_irq_en(irq2_mask & 0x01 != 0);
        })
        .await
    }

    #[bisync]
    pub async fn get_interrupt_status(&mut self) -> Result<(u8,), AxpError<I2CBusErr>> {
        // Read IRQ status register 0
        let mut op0 = self.ll.irq_status_0();
        let status0 = read_internal(&mut op0).await?;

        let irq0 = (status0.soc_warning_level2_irq() as u8) << 7
            | (status0.soc_warning_level1_irq() as u8) << 6
            | (status0.gauge_watchdog_timeout_irq() as u8) << 5
            | (status0.new_soc_irq() as u8) << 4
            | (status0.battery_charge_over_temp_irq() as u8) << 3
            | (status0.battery_charge_under_temp_irq() as u8) << 2
            | (status0.battery_work_over_temp_irq() as u8) << 1
            | (status0.battery_work_under_temp_irq() as u8);

        Ok((irq0,))
    }

    #[bisync]
    pub async fn get_interrupt_status2(&mut self) -> Result<(u8,), AxpError<I2CBusErr>> {
        let mut op2 = self.ll.irq_status_2();
        let status2 = read_internal(&mut op2).await?;

        let irq2 = (status2.wdexp_irq() as u8) << 7
            | (status2.ldooc_irq() as u8) << 6
            | (status2.bocp_irq() as u8) << 5
            | (status2.chgdn_irq() as u8) << 4
            | (status2.chgst_irq() as u8) << 3
            | (status2.dot11_irq() as u8) << 2
            | (status2.chgte_irq() as u8) << 1
            | (status2.bovp_irq() as u8);

        Ok((irq2,))
    }

    #[bisync]
    pub async fn clear_interrupt_status(&mut self) -> Result<(), AxpError<I2CBusErr>> {
        // Clear all interrupt status bits by writing 1 to them
        let mut op0 = self.ll.irq_status_0();
        write_internal(&mut op0, |r| {
            r.set_soc_warning_level2_irq(true);
            r.set_soc_warning_level1_irq(true);
            r.set_gauge_watchdog_timeout_irq(true);
            r.set_new_soc_irq(true);
            r.set_battery_charge_over_temp_irq(true);
            r.set_battery_charge_under_temp_irq(true);
            r.set_battery_work_over_temp_irq(true);
            r.set_battery_work_under_temp_irq(true);
        })
        .await
    }

    #[bisync]
    pub async fn clear_interrupt_status2(&mut self) -> Result<(), AxpError<I2CBusErr>> {
        let mut op2 = self.ll.irq_status_2();
        write_internal(&mut op2, |r| {
            r.set_wdexp_irq(true);
            r.set_ldooc_irq(true);
            r.set_bocp_irq(true);
            r.set_chgdn_irq(true);
            r.set_chgst_irq(true);
            r.set_dot11_irq(true);
            r.set_chgte_irq(true);
            r.set_bovp_irq(true);
        })
        .await
    }
}
