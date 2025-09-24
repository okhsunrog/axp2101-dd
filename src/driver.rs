use super::{I2c, RegisterInterface, bisync, only_async, only_sync};
use crate::{AXP2101_I2C_ADDRESS, AxpError, AxpInterface, AxpLowLevel, DcId, LdoId};
use crate::{BatteryCurrentDirection, FastChargeCurrentLimit, ChargeVoltageLimit};

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
    pub async fn get_battery_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        // Read 16-bit battery voltage ADC register
        let mut op = self.ll.battery_voltage_adc();
        let adc_data = read_internal(&mut op).await?;
        
        // Extract 14-bit value from 16-bit register
        let adc_value = adc_data.raw_value();
        // AXP2101 battery voltage ADC: 14-bit, LSB = 1mV
        Ok(adc_value as f32)
    }

    #[bisync]
    pub async fn get_vbus_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        // Read 16-bit VBUS voltage ADC register
        let mut op = self.ll.vbus_voltage_adc();
        let adc_data = read_internal(&mut op).await?;
        
        // Extract 14-bit value from 16-bit register
        let adc_value = adc_data.raw_value();
        // AXP2101 VBUS voltage ADC: 14-bit, LSB = 1mV  
        Ok(adc_value as f32)
    }

    #[bisync]
    pub async fn get_die_temperature_c(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        // Read 16-bit internal temperature ADC register
        let mut op = self.ll.internal_temperature_adc();
        let adc_data = read_internal(&mut op).await?;
        
        // Extract 14-bit value from 16-bit register
        let adc_value = adc_data.raw_value();
        // AXP2101 temperature ADC conversion to Celsius
        Ok(-144.7 + (adc_value as f32 * 0.1))
    }

    #[bisync]
    pub async fn set_dcdc_enable(
        &mut self,
        dc: DcId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.dcdc_config_0();
        match dc {
            DcId::Dcdc1 => {
                modify_internal(&mut op, |r| r.set_dcdc1_enable(enable)).await
            }
            DcId::Dcdc2 => {
                modify_internal(&mut op, |r| r.set_dcdc2_enable(enable)).await
            }
            DcId::Dcdc3 => {
                modify_internal(&mut op, |r| r.set_dcdc3_enable(enable)).await
            }
            DcId::Dcdc4 => {
                modify_internal(&mut op, |r| r.set_dcdc4_enable(enable)).await
            }
            DcId::Dcdc5 => {
                modify_internal(&mut op, |r| r.set_dcdc5_enable(enable)).await
            }
        }
    }

    #[bisync]
    pub async fn set_dcdc_voltage(
        &mut self,
        dc: DcId,
        voltage_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let voltage_val = match dc {
            DcId::Dcdc1 => {
                // DCDC1: 1.5V-3.4V, 10mV/step up to 3.2V, then 20mV/step
                if !(1500..=3400).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                if voltage_mv <= 3200 {
                    ((voltage_mv - 1500) / 10) as u8
                } else {
                    (170 + (voltage_mv - 3200) / 20) as u8
                }
            }
            DcId::Dcdc2 | DcId::Dcdc3 => {
                // DCDC2/3: 0.5V-1.2V or 1.22V-1.54V, 10mV/step
                if voltage_mv <= 1200 && voltage_mv >= 500 {
                    ((voltage_mv - 500) / 10) as u8
                } else if (1220..=1540).contains(&voltage_mv) {
                    (70 + (voltage_mv - 1220) / 10) as u8
                } else {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
            }
            DcId::Dcdc4 => {
                // DCDC4: 0.5V-1.2V or 1.22V-3.4V, 10mV/step
                if voltage_mv <= 1200 && voltage_mv >= 500 {
                    ((voltage_mv - 500) / 10) as u8
                } else if (1220..=3400).contains(&voltage_mv) {
                    (70 + (voltage_mv - 1220) / 10) as u8
                } else {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
            }
            DcId::Dcdc5 => {
                return Err(AxpError::NotSupported("DCDC5 not available on AXP2101"));
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
                Err(AxpError::NotSupported("DCDC5 not available on AXP2101"))
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
                // DLDO2 might be on a different register - need to check
                Err(AxpError::NotSupported("DLDO2 enable not implemented yet"))
            },
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
                    LdoId::Dldo2 => unreachable!(), // Already handled above
                }).await
            }
        }
    }

    #[bisync]
    pub async fn set_ldo_voltage_mv(
        &mut self,
        _ldo: LdoId,
        _voltage_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        // TODO: LDO voltage control registers not yet implemented in device.yaml
        // Need to add voltage configuration registers for each LDO
        Err(AxpError::NotSupported("LDO voltage control not implemented yet"))
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
        // Combine high and low chip ID bits to get full chip ID
        let chip_id = (chip_data.chip_id_high() << 4) | chip_data.chip_id_low();
        Ok(chip_id)
    }

    #[bisync]
    pub async fn get_power_status(&mut self) -> Result<(bool, bool, bool, bool), AxpError<I2CBusErr>> {
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
            matches!(system_status.battery_current_direction(), BatteryCurrentDirection::Charging),
        ))
    }

    #[bisync]
    pub async fn enable_adc_channel(
        &mut self,
        _channel_mask: u8,
    ) -> Result<(), AxpError<I2CBusErr>> {
        // TODO: ADC channel enable registers not yet implemented in device.yaml
        // Need to add ADC enable control registers
        Err(AxpError::NotSupported("ADC channel enable not implemented yet"))
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
        }).await?;

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
        }).await
    }

    #[bisync]
    pub async fn get_interrupt_status(&mut self) -> Result<(u8,), AxpError<I2CBusErr>> {
        // Read IRQ status register 0 
        let mut op0 = self.ll.irq_status_0();
        let status0 = read_internal(&mut op0).await?;
        
        let irq0 = (status0.soc_warning_level2_irq() as u8) << 7 |
                   (status0.soc_warning_level1_irq() as u8) << 6 |
                   (status0.gauge_watchdog_timeout_irq() as u8) << 5 |
                   (status0.new_soc_irq() as u8) << 4 |
                   (status0.battery_charge_over_temp_irq() as u8) << 3 |
                   (status0.battery_charge_under_temp_irq() as u8) << 2 |
                   (status0.battery_work_over_temp_irq() as u8) << 1 |
                   (status0.battery_work_under_temp_irq() as u8);
        
        Ok((irq0,))
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
        }).await
    }
}