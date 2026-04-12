#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calc {
    #[doc = "Calculate a CRC-32 (IEEE802.3 polynomial)."]
    Crc32 = 0x0,
    #[doc = "Calculate a CRC-32 (IEEE802.3 polynomial) with bit reversed data."]
    Crc32r = 0x01,
    #[doc = "Calculate a CRC-16-CCITT."]
    Crc16 = 0x02,
    #[doc = "Calculate a CRC-16-CCITT with bit reversed data."]
    Crc16r = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "XOR reduction over all data. == 1 if the total 1 population count is odd."]
    Even = 0x0e,
    #[doc = "Calculate a simple 32-bit checksum (addition with a 32 bit accumulator)."]
    Sum = 0x0f,
}
impl Calc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calc {
    #[inline(always)]
    fn from(val: u8) -> Calc {
        Calc::from_bits(val)
    }
}
impl From<Calc> for u8 {
    #[inline(always)]
    fn from(val: Calc) -> u8 {
        Calc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataSize {
    SizeByte = 0x0,
    SizeHalfword = 0x01,
    SizeWord = 0x02,
    _RESERVED_3 = 0x03,
}
impl DataSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DataSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DataSize {
    #[inline(always)]
    fn from(val: u8) -> DataSize {
        DataSize::from_bits(val)
    }
}
impl From<DataSize> for u8 {
    #[inline(always)]
    fn from(val: DataSize) -> u8 {
        DataSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TransCountMode {
    Normal = 0x0,
    TriggerSelf = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    Endless = 0x0f,
}
impl TransCountMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TransCountMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TransCountMode {
    #[inline(always)]
    fn from(val: u8) -> TransCountMode {
        TransCountMode::from_bits(val)
    }
}
impl From<TransCountMode> for u8 {
    #[inline(always)]
    fn from(val: TransCountMode) -> u8 {
        TransCountMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TreqSel {
    #[doc = "Select PIO0's TX FIFO 0 as TREQ."]
    Pio0Tx0 = 0x0,
    #[doc = "Select PIO0's TX FIFO 1 as TREQ."]
    Pio0Tx1 = 0x01,
    #[doc = "Select PIO0's TX FIFO 2 as TREQ."]
    Pio0Tx2 = 0x02,
    #[doc = "Select PIO0's TX FIFO 3 as TREQ."]
    Pio0Tx3 = 0x03,
    #[doc = "Select PIO0's RX FIFO 0 as TREQ."]
    Pio0Rx0 = 0x04,
    #[doc = "Select PIO0's RX FIFO 1 as TREQ."]
    Pio0Rx1 = 0x05,
    #[doc = "Select PIO0's RX FIFO 2 as TREQ."]
    Pio0Rx2 = 0x06,
    #[doc = "Select PIO0's RX FIFO 3 as TREQ."]
    Pio0Rx3 = 0x07,
    #[doc = "Select PIO1's TX FIFO 0 as TREQ."]
    Pio1Tx0 = 0x08,
    #[doc = "Select PIO1's TX FIFO 1 as TREQ."]
    Pio1Tx1 = 0x09,
    #[doc = "Select PIO1's TX FIFO 2 as TREQ."]
    Pio1Tx2 = 0x0a,
    #[doc = "Select PIO1's TX FIFO 3 as TREQ."]
    Pio1Tx3 = 0x0b,
    #[doc = "Select PIO1's RX FIFO 0 as TREQ."]
    Pio1Rx0 = 0x0c,
    #[doc = "Select PIO1's RX FIFO 1 as TREQ."]
    Pio1Rx1 = 0x0d,
    #[doc = "Select PIO1's RX FIFO 2 as TREQ."]
    Pio1Rx2 = 0x0e,
    #[doc = "Select PIO1's RX FIFO 3 as TREQ."]
    Pio1Rx3 = 0x0f,
    #[doc = "Select PIO2's TX FIFO 0 as TREQ."]
    Pio2Tx0 = 0x10,
    #[doc = "Select PIO2's TX FIFO 1 as TREQ."]
    Pio2Tx1 = 0x11,
    #[doc = "Select PIO2's TX FIFO 2 as TREQ."]
    Pio2Tx2 = 0x12,
    #[doc = "Select PIO2's TX FIFO 3 as TREQ."]
    Pio2Tx3 = 0x13,
    #[doc = "Select PIO2's RX FIFO 0 as TREQ."]
    Pio2Rx0 = 0x14,
    #[doc = "Select PIO2's RX FIFO 1 as TREQ."]
    Pio2Rx1 = 0x15,
    #[doc = "Select PIO2's RX FIFO 2 as TREQ."]
    Pio2Rx2 = 0x16,
    #[doc = "Select PIO2's RX FIFO 3 as TREQ."]
    Pio2Rx3 = 0x17,
    #[doc = "Select SPI0's TX FIFO as TREQ."]
    Spi0Tx = 0x18,
    #[doc = "Select SPI0's RX FIFO as TREQ."]
    Spi0Rx = 0x19,
    #[doc = "Select SPI1's TX FIFO as TREQ."]
    Spi1Tx = 0x1a,
    #[doc = "Select SPI1's RX FIFO as TREQ."]
    Spi1Rx = 0x1b,
    #[doc = "Select UART0's TX FIFO as TREQ."]
    Uart0Tx = 0x1c,
    #[doc = "Select UART0's RX FIFO as TREQ."]
    Uart0Rx = 0x1d,
    #[doc = "Select UART1's TX FIFO as TREQ."]
    Uart1Tx = 0x1e,
    #[doc = "Select UART1's RX FIFO as TREQ."]
    Uart1Rx = 0x1f,
    #[doc = "Select PWM Counter 0's Wrap Value as TREQ."]
    PwmWrap0 = 0x20,
    #[doc = "Select PWM Counter 1's Wrap Value as TREQ."]
    PwmWrap1 = 0x21,
    #[doc = "Select PWM Counter 2's Wrap Value as TREQ."]
    PwmWrap2 = 0x22,
    #[doc = "Select PWM Counter 3's Wrap Value as TREQ."]
    PwmWrap3 = 0x23,
    #[doc = "Select PWM Counter 4's Wrap Value as TREQ."]
    PwmWrap4 = 0x24,
    #[doc = "Select PWM Counter 5's Wrap Value as TREQ."]
    PwmWrap5 = 0x25,
    #[doc = "Select PWM Counter 6's Wrap Value as TREQ."]
    PwmWrap6 = 0x26,
    #[doc = "Select PWM Counter 7's Wrap Value as TREQ."]
    PwmWrap7 = 0x27,
    #[doc = "Select PWM Counter 8's Wrap Value as TREQ."]
    PwmWrap8 = 0x28,
    #[doc = "Select PWM Counter 9's Wrap Value as TREQ."]
    PwmWrap9 = 0x29,
    #[doc = "Select PWM Counter 0's Wrap Value as TREQ."]
    PwmWrap10 = 0x2a,
    #[doc = "Select PWM Counter 1's Wrap Value as TREQ."]
    PwmWrap11 = 0x2b,
    #[doc = "Select I2C0's TX FIFO as TREQ."]
    I2c0Tx = 0x2c,
    #[doc = "Select I2C0's RX FIFO as TREQ."]
    I2c0Rx = 0x2d,
    #[doc = "Select I2C1's TX FIFO as TREQ."]
    I2c1Tx = 0x2e,
    #[doc = "Select I2C1's RX FIFO as TREQ."]
    I2c1Rx = 0x2f,
    #[doc = "Select the ADC as TREQ."]
    Adc = 0x30,
    #[doc = "Select the XIP Streaming FIFO as TREQ."]
    XipStream = 0x31,
    #[doc = "Select XIP_QMITX as TREQ."]
    XipQmitx = 0x32,
    #[doc = "Select XIP_QMIRX as TREQ."]
    XipQmirx = 0x33,
    #[doc = "Select HSTX as TREQ."]
    Hstx = 0x34,
    #[doc = "Select CORESIGHT as TREQ."]
    Coresight = 0x35,
    #[doc = "Select SHA256 as TREQ."]
    Sha256 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    #[doc = "Select Timer 0 as TREQ."]
    Timer0 = 0x3b,
    #[doc = "Select Timer 1 as TREQ."]
    Timer1 = 0x3c,
    #[doc = "Select Timer 2 as TREQ (Optional)."]
    Timer2 = 0x3d,
    #[doc = "Select Timer 3 as TREQ (Optional)."]
    Timer3 = 0x3e,
    #[doc = "Permanent request, for unpaced transfers."]
    Permanent = 0x3f,
}
impl TreqSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TreqSel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TreqSel {
    #[inline(always)]
    fn from(val: u8) -> TreqSel {
        TreqSel::from_bits(val)
    }
}
impl From<TreqSel> for u8 {
    #[inline(always)]
    fn from(val: TreqSel) -> u8 {
        TreqSel::to_bits(val)
    }
}
