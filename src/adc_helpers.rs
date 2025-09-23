/// Helper to extract a 12-bit ADC value from a raw u16 read over two 8-bit registers.
/// Assumes Big Endian read: raw_u16 = (MSB_register_byte << 8) | LSB_register_byte.
/// MSB_register_byte contains ADC[11:4].
/// LSB_register_byte contains ADC[3:0] in its lower nibble.
pub(crate) fn adc_12bit_from_raw_u16(raw_be_u16: u16) -> u16 {
    let msb_reg_val = (raw_be_u16 >> 8) as u8; // Content of the first physical register (e.g., 0x78)
    let lsb_reg_val = (raw_be_u16 & 0xFF) as u8; // Content of the second physical register (e.g., 0x79)
    ((msb_reg_val as u16) << 4) | ((lsb_reg_val & 0x0F) as u16)
}

/// Helper to extract a 13-bit ADC value from a raw u16 read over two 8-bit registers.
/// Assumes Big Endian read: raw_u16 = (MSB_register_byte << 8) | LSB_register_byte.
/// MSB_register_byte contains ADC[12:5].
/// LSB_register_byte contains ADC[4:0] in its bits 4:0.
pub(crate) fn adc_13bit_from_raw_u16(raw_be_u16: u16) -> u16 {
    let msb_reg_val = (raw_be_u16 >> 8) as u8; // Content of the first physical register (e.g., 0x7A)
    let lsb_reg_val = (raw_be_u16 & 0xFF) as u8; // Content of the second physical register (e.g., 0x7B)
    ((msb_reg_val as u16) << 5) | ((lsb_reg_val & 0x1F) as u16)
}

/// Helper to extract a 24-bit ADC value from a raw u32 read over three 8-bit registers.
/// Assumes Big Endian read: raw_u32 = (MSB_reg << 16) | (MID_reg << 8) | LSB_reg.
/// The device-driver field for a 24-bit uint will likely yield a u32 with data in lower 24 bits.
pub(crate) fn adc_24bit_from_raw_u32(raw_be_u32: u32) -> u32 {
    raw_be_u32 & 0x00FFFFFF // Ensure only the lower 24 bits are used
}