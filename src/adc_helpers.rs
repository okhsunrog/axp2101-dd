/// Helper to combine high and low bytes into a 14-bit ADC value.
/// AXP2101 ADC format:
/// - High byte: bits [5:0] contain ADC[13:8]
/// - Low byte: bits [7:0] contain ADC[7:0]
///   Returns a 14-bit value (0-16383).
#[inline]
pub(crate) fn adc_14bit_combine(high_6bit: u8, low_8bit: u8) -> u16 {
    ((high_6bit as u16) << 8) | (low_8bit as u16)
}

/// Convert battery voltage ADC value to millivolts.
/// AXP2101 battery voltage ADC: 14-bit, 1mV per LSB.
#[inline]
pub(crate) fn battery_voltage_to_mv(adc_value: u16) -> u16 {
    adc_value
}

/// Convert VBUS voltage ADC value to millivolts.
/// AXP2101 VBUS voltage ADC: 14-bit, 1mV per LSB.
#[inline]
pub(crate) fn vbus_voltage_to_mv(adc_value: u16) -> u16 {
    adc_value
}

/// Convert VSYS voltage ADC value to millivolts.
/// AXP2101 VSYS voltage ADC: 14-bit, 1mV per LSB.
#[inline]
pub(crate) fn vsys_voltage_to_mv(adc_value: u16) -> u16 {
    adc_value
}

/// Convert TS pin ADC value to millivolts.
/// AXP2101 TS pin ADC: 14-bit, 0.5mV per LSB.
#[inline]
pub(crate) fn ts_pin_to_mv(adc_value: u16) -> f32 {
    adc_value as f32 * 0.5
}

/// Convert internal temperature ADC value to degrees Celsius.
/// AXP2101 die temperature formula: T = -144.7 + (adc_value * 0.1)
/// LSB = 0.1°C, offset = -144.7°C
#[inline]
pub(crate) fn die_temp_to_celsius(adc_value: u16) -> f32 {
    -144.7 + (adc_value as f32 * 0.1)
}
