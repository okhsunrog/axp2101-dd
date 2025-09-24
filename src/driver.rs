use super::{I2c, RegisterInterface, bisync, only_async, only_sync};
use crate::{AXP2101_I2C_ADDRESS, AxpError, AxpInterface, AxpLowLevel, DcId, LdoId};

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
        // Read high byte first
        let mut op_h = self.ll.bat_voltage_adc_h();
        let adc_h = read_internal(&mut op_h).await?;
        
        // Then read low byte  
        let mut op_l = self.ll.bat_voltage_adc_l();
        let adc_l = read_internal(&mut op_l).await?;
        
        let adc_value = ((adc_h.bat_vol_adc_h() as u16) << 6) | (adc_l.bat_vol_adc_l() as u16);
        // AXP2101 battery voltage ADC: 14-bit, LSB = 1mV
        Ok(adc_value as f32)
    }

    #[bisync]
    pub async fn get_vbus_voltage_mv(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let mut op_h = self.ll.vbus_voltage_adc_h();
        let adc_h = read_internal(&mut op_h).await?;
        
        let mut op_l = self.ll.vbus_voltage_adc_l();
        let adc_l = read_internal(&mut op_l).await?;
        
        let adc_value = ((adc_h.vbus_vol_adc_h() as u16) << 6) | (adc_l.vbus_vol_adc_l() as u16);
        // AXP2101 VBUS voltage ADC: 14-bit, LSB = 1mV  
        Ok(adc_value as f32)
    }

    #[bisync]
    pub async fn get_die_temperature_c(&mut self) -> Result<f32, AxpError<I2CBusErr>> {
        let mut op_h = self.ll.die_temperature_adc_h();
        let adc_h = read_internal(&mut op_h).await?;
        
        let mut op_l = self.ll.die_temperature_adc_l();
        let adc_l = read_internal(&mut op_l).await?;
        
        let adc_value = ((adc_h.die_temp_adc_h() as u16) << 6) | (adc_l.die_temp_adc_l() as u16);
        // AXP2101 temperature ADC conversion to Celsius
        Ok(-144.7 + (adc_value as f32 * 0.1))
    }

    #[bisync]
    pub async fn set_dcdc_enable(
        &mut self,
        dc: DcId,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        match dc {
            DcId::Dcdc1 => {
                let mut op = self.ll.power_on_off_control();
                modify_internal(&mut op, |r| r.set_dcdc_1_enable(enable)).await
            }
            DcId::Dcdc2 => {
                let mut op = self.ll.dcdc_control();
                modify_internal(&mut op, |r| r.set_dcdc_2_enable(enable)).await
            }
            DcId::Dcdc3 => {
                let mut op = self.ll.dcdc_control();
                modify_internal(&mut op, |r| r.set_dcdc_3_enable(enable)).await
            }
            DcId::Dcdc4 => {
                let mut op = self.ll.dcdc_control();
                modify_internal(&mut op, |r| r.set_dcdc_4_enable(enable)).await
            }
            DcId::Dcdc5 => {
                Err(AxpError::NotSupported("DCDC5 not available on AXP2101"))
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
                let mut op = self.ll.dcdc_1_voltage_control();
                modify_internal(&mut op, |r| r.set_dcdc_1_voltage(voltage_val)).await
            }
            DcId::Dcdc2 => {
                let mut op = self.ll.dcdc_2_voltage_control();
                modify_internal(&mut op, |r| r.set_dcdc_2_voltage(voltage_val)).await
            }
            DcId::Dcdc3 => {
                let mut op = self.ll.dcdc_3_voltage_control();
                modify_internal(&mut op, |r| r.set_dcdc_3_voltage(voltage_val)).await
            }
            DcId::Dcdc4 => {
                let mut op = self.ll.dcdc_4_voltage_control();
                modify_internal(&mut op, |r| r.set_dcdc_4_voltage(voltage_val)).await
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
            LdoId::Aldo1 | LdoId::Aldo2 | LdoId::Aldo3 | LdoId::Aldo4 | LdoId::Bldo1 | LdoId::Bldo2 => {
                let mut op = self.ll.ldo_control_1();
                modify_internal(&mut op, |r| match ldo {
                    LdoId::Aldo1 => r.set_aldo_1_enable(enable),
                    LdoId::Aldo2 => r.set_aldo_2_enable(enable),
                    LdoId::Aldo3 => r.set_aldo_3_enable(enable),
                    LdoId::Aldo4 => r.set_aldo_4_enable(enable),
                    LdoId::Bldo1 => r.set_bldo_1_enable(enable),
                    LdoId::Bldo2 => r.set_bldo_2_enable(enable),
                    _ => unreachable!(),
                }).await
            }
            LdoId::Dldo1 | LdoId::Dldo2 | LdoId::Cpusldo => {
                let mut op = self.ll.ldo_control_2();
                modify_internal(&mut op, |r| match ldo {
                    LdoId::Dldo1 => r.set_dldo_1_enable(enable),
                    LdoId::Dldo2 => r.set_dldo_2_enable(enable),
                    LdoId::Cpusldo => r.set_cpusldo_enable(enable),
                    _ => unreachable!(),
                }).await
            }
        }
    }

    #[bisync]
    pub async fn set_ldo_voltage_mv(
        &mut self,
        ldo: LdoId,
        voltage_mv: u16,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let voltage_val = match ldo {
            LdoId::Aldo1 | LdoId::Aldo2 | LdoId::Aldo3 | LdoId::Aldo4 => {
                // ALDO1-4: 0.5V-3.5V, 100mV/step
                if !(500..=3500).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 100) as u8;
                if val > 31 {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                val
            }
            LdoId::Bldo1 | LdoId::Bldo2 => {
                // BLDO1-2: 0.5V-3.5V, 100mV/step  
                if !(500..=3500).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 100) as u8;
                if val > 31 {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                val
            }
            LdoId::Dldo1 => {
                // DLDO1: 0.5V-3.3V, 100mV/step
                if !(500..=3300).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 100) as u8;
                if val > 28 {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                val
            }
            LdoId::Dldo2 | LdoId::Cpusldo => {
                // DLDO2, CPUSLDO: 0.5V-1.4V, 50mV/step
                if !(500..=1400).contains(&voltage_mv) {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                let val = ((voltage_mv - 500) / 50) as u8;
                if val > 18 {
                    return Err(AxpError::InvalidVoltage(voltage_mv));
                }
                val
            }
        };

        match ldo {
            LdoId::Aldo1 => {
                let mut op = self.ll.aldo_1_voltage_control();
                modify_internal(&mut op, |r| r.set_aldo_1_voltage(voltage_val)).await
            }
            LdoId::Aldo2 => {
                let mut op = self.ll.aldo_2_voltage_control();
                modify_internal(&mut op, |r| r.set_aldo_2_voltage(voltage_val)).await
            }
            LdoId::Aldo3 => {
                let mut op = self.ll.aldo_3_voltage_control();
                modify_internal(&mut op, |r| r.set_aldo_3_voltage(voltage_val)).await
            }
            LdoId::Aldo4 => {
                let mut op = self.ll.aldo_4_voltage_control();
                modify_internal(&mut op, |r| r.set_aldo_4_voltage(voltage_val)).await
            }
            LdoId::Bldo1 => {
                let mut op = self.ll.bldo_1_voltage_control();
                modify_internal(&mut op, |r| r.set_bldo_1_voltage(voltage_val)).await
            }
            LdoId::Bldo2 => {
                let mut op = self.ll.bldo_2_voltage_control();
                modify_internal(&mut op, |r| r.set_bldo_2_voltage(voltage_val)).await
            }
            LdoId::Dldo1 => {
                let mut op = self.ll.dldo_1_voltage_control();
                modify_internal(&mut op, |r| r.set_dldo_1_voltage(voltage_val)).await
            }
            LdoId::Dldo2 => {
                let mut op = self.ll.dldo_2_voltage_control();
                modify_internal(&mut op, |r| r.set_dldo_2_voltage(voltage_val)).await
            }
            LdoId::Cpusldo => {
                let mut op = self.ll.cpusldo_voltage_control();
                modify_internal(&mut op, |r| r.set_cpusldo_voltage(voltage_val)).await
            }
        }
    }

    #[bisync]
    pub async fn set_battery_charge_enable(
        &mut self,
        enable: bool,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.charge_control_1();
        modify_internal(&mut op, |r| r.set_chg_enable(enable)).await
    }

    #[bisync]
    pub async fn set_battery_charge_current(
        &mut self,
        current_setting: u8,
    ) -> Result<(), AxpError<I2CBusErr>> {
        if current_setting > 3 {
            return Err(AxpError::InvalidCurrent(current_setting as u16));
        }
        let mut op = self.ll.charge_control_1();
        modify_internal(&mut op, |r| r.set_chg_current(current_setting)).await
    }

    #[bisync]
    pub async fn set_battery_charge_voltage(
        &mut self,
        voltage_setting: u8,
    ) -> Result<(), AxpError<I2CBusErr>> {
        if voltage_setting > 3 {
            return Err(AxpError::InvalidVoltage(voltage_setting as u16));
        }
        let mut op = self.ll.charge_control_1();
        modify_internal(&mut op, |r| r.set_chg_target_voltage(voltage_setting)).await
    }

    #[bisync]
    pub async fn get_chip_id(&mut self) -> Result<u8, AxpError<I2CBusErr>> {
        let mut op = self.ll.chip_id();
        let chip_data = read_internal(&mut op).await?;
        Ok(chip_data.chip_id())
    }

    #[bisync]
    pub async fn get_power_status(&mut self) -> Result<(bool, bool, bool, bool), AxpError<I2CBusErr>> {
        let mut op = self.ll.power_status_input();
        let status = read_internal(&mut op).await?;
        
        Ok((
            status.acin_present(),
            status.acin_available(),
            status.vbus_present(),
            status.vbus_available(),
        ))
    }

    #[bisync]
    pub async fn get_battery_status(&mut self) -> Result<(bool, bool), AxpError<I2CBusErr>> {
        let mut op = self.ll.power_status_battery();
        let status = read_internal(&mut op).await?;
        
        Ok((
            status.battery_present(),
            status.charging_indication(),
        ))
    }

    #[bisync]
    pub async fn enable_adc_channel(
        &mut self,
        channel_mask: u8,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op = self.ll.adc_channel_enable();
        write_internal(&mut op, |r| {
            if channel_mask & 0x80 != 0 { r.set_ts_adc_en(true); }
            if channel_mask & 0x40 != 0 { r.set_vbus_adc_en(true); }
            if channel_mask & 0x20 != 0 { r.set_acin_adc_en(true); }
            if channel_mask & 0x10 != 0 { r.set_bat_adc_en(true); }
            if channel_mask & 0x08 != 0 { r.set_vsys_adc_en(true); }
            if channel_mask & 0x04 != 0 { r.set_die_temp_adc_en(true); }
        }).await
    }

    #[bisync]
    pub async fn enable_interrupts(
        &mut self,
        irq1_mask: u8,
        irq2_mask: u8,
        irq3_mask: u8,
    ) -> Result<(), AxpError<I2CBusErr>> {
        let mut op1 = self.ll.irq_enable_1();
        write_internal(&mut op1, |r| {
            r.set_socwarn_2_irq_en(irq1_mask & 0x80 != 0);
            r.set_socwarn_1_irq_en(irq1_mask & 0x40 != 0);
            r.set_gwdt_irq_en(irq1_mask & 0x20 != 0);
            r.set_newsoc_irq_en(irq1_mask & 0x10 != 0);
            r.set_bat_over_irq_en(irq1_mask & 0x08 != 0);
            r.set_bat_under_irq_en(irq1_mask & 0x04 != 0);
            r.set_socwarn_0_irq_en(irq1_mask & 0x02 != 0);
            r.set_percent_irq_en(irq1_mask & 0x01 != 0);
        }).await?;

        let mut op2 = self.ll.irq_enable_2();
        write_internal(&mut op2, |r| {
            r.set_pkey_negative_irq_en(irq2_mask & 0x80 != 0);
            r.set_pkey_positive_irq_en(irq2_mask & 0x40 != 0);
            r.set_wdt_irq_en(irq2_mask & 0x20 != 0);
            r.set_pok_fall_irq_en(irq2_mask & 0x10 != 0);
            r.set_pok_rise_irq_en(irq2_mask & 0x08 != 0);
            r.set_dcdc_under_irq_en(irq2_mask & 0x04 != 0);
        }).await?;

        let mut op3 = self.ll.irq_enable_3();
        write_internal(&mut op3, |r| {
            r.set_die_ot_alm_irq_en(irq3_mask & 0x80 != 0);
            r.set_chg_done_irq_en(irq3_mask & 0x40 != 0);
            r.set_chg_irq_en(irq3_mask & 0x20 != 0);
            r.set_bat_in_irq_en(irq3_mask & 0x10 != 0);
            r.set_bat_re_irq_en(irq3_mask & 0x08 != 0);
            r.set_vbus_in_irq_en(irq3_mask & 0x04 != 0);
            r.set_vbus_re_irq_en(irq3_mask & 0x02 != 0);
            r.set_acin_over_irq_en(irq3_mask & 0x01 != 0);
        }).await
    }

    #[bisync]
    pub async fn get_interrupt_status(&mut self) -> Result<(u8, u8, u8), AxpError<I2CBusErr>> {
        let mut op1 = self.ll.irq_status_1();
        let status1 = read_internal(&mut op1).await?;
        
        let mut op2 = self.ll.irq_status_2();
        let status2 = read_internal(&mut op2).await?;
        
        let mut op3 = self.ll.irq_status_3();
        let status3 = read_internal(&mut op3).await?;
        
        let irq1 = (status1.socwarn_2_irq() as u8) << 7 |
                   (status1.socwarn_1_irq() as u8) << 6 |
                   (status1.gwdt_irq() as u8) << 5 |
                   (status1.newsoc_irq() as u8) << 4 |
                   (status1.bat_over_irq() as u8) << 3 |
                   (status1.bat_under_irq() as u8) << 2 |
                   (status1.socwarn_0_irq() as u8) << 1 |
                   (status1.percent_irq() as u8);
                   
        let irq2 = (status2.pkey_negative_irq() as u8) << 7 |
                   (status2.pkey_positive_irq() as u8) << 6 |
                   (status2.wdt_irq() as u8) << 5 |
                   (status2.pok_fall_irq() as u8) << 4 |
                   (status2.pok_rise_irq() as u8) << 3 |
                   (status2.dcdc_under_irq() as u8) << 2;
                   
        let irq3 = (status3.die_ot_alm_irq() as u8) << 7 |
                   (status3.chg_done_irq() as u8) << 6 |
                   (status3.chg_irq() as u8) << 5 |
                   (status3.bat_in_irq() as u8) << 4 |
                   (status3.bat_re_irq() as u8) << 3 |
                   (status3.vbus_in_irq() as u8) << 2 |
                   (status3.vbus_re_irq() as u8) << 1 |
                   (status3.acin_over_irq() as u8);
        
        Ok((irq1, irq2, irq3))
    }

    #[bisync]
    pub async fn clear_interrupt_status(&mut self) -> Result<(), AxpError<I2CBusErr>> {
        // Clear all interrupt status bits by writing 1 to them
        let mut op1 = self.ll.irq_status_1();
        write_internal(&mut op1, |r| {
            r.set_socwarn_2_irq(true);
            r.set_socwarn_1_irq(true);
            r.set_gwdt_irq(true);
            r.set_newsoc_irq(true);
            r.set_bat_over_irq(true);
            r.set_bat_under_irq(true);
            r.set_socwarn_0_irq(true);
            r.set_percent_irq(true);
        }).await?;

        let mut op2 = self.ll.irq_status_2();
        write_internal(&mut op2, |r| {
            r.set_pkey_negative_irq(true);
            r.set_pkey_positive_irq(true);
            r.set_wdt_irq(true);
            r.set_pok_fall_irq(true);
            r.set_pok_rise_irq(true);
            r.set_dcdc_under_irq(true);
        }).await?;

        let mut op3 = self.ll.irq_status_3();
        write_internal(&mut op3, |r| {
            r.set_die_ot_alm_irq(true);
            r.set_chg_done_irq(true);
            r.set_chg_irq(true);
            r.set_bat_in_irq(true);
            r.set_bat_re_irq(true);
            r.set_vbus_in_irq(true);
            r.set_vbus_re_irq(true);
            r.set_acin_over_irq(true);
        }).await
    }
}