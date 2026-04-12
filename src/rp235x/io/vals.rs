#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0CtrlFuncsel {
    JtagTck = 0x0,
    Spi0Rx = 0x01,
    Uart0Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA0 = 0x04,
    SiobProc0 = 0x05,
    Pio00 = 0x06,
    Pio10 = 0x07,
    Pio20 = 0x08,
    XipSsN1 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio0CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio0CtrlFuncsel {
        Gpio0CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio0CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio0CtrlFuncsel) -> u8 {
        Gpio0CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio10CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Sclk = 0x01,
    Uart1Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA5 = 0x04,
    SiobProc10 = 0x05,
    Pio010 = 0x06,
    Pio110 = 0x07,
    Pio210 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    Uart1Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio10CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio10CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio10CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio10CtrlFuncsel {
        Gpio10CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio10CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio10CtrlFuncsel) -> u8 {
        Gpio10CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio11CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Tx = 0x01,
    Uart1Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB5 = 0x04,
    SiobProc11 = 0x05,
    Pio011 = 0x06,
    Pio111 = 0x07,
    Pio211 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    Uart1Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio11CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio11CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio11CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio11CtrlFuncsel {
        Gpio11CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio11CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio11CtrlFuncsel) -> u8 {
        Gpio11CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio12CtrlFuncsel {
    Hstx0 = 0x0,
    Spi1Rx = 0x01,
    Uart0Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA6 = 0x04,
    SiobProc12 = 0x05,
    Pio012 = 0x06,
    Pio112 = 0x07,
    Pio212 = 0x08,
    ClocksGpin0 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio12CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio12CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio12CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio12CtrlFuncsel {
        Gpio12CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio12CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio12CtrlFuncsel) -> u8 {
        Gpio12CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio13CtrlFuncsel {
    Hstx1 = 0x0,
    Spi1SsN = 0x01,
    Uart0Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB6 = 0x04,
    SiobProc13 = 0x05,
    Pio013 = 0x06,
    Pio113 = 0x07,
    Pio213 = 0x08,
    ClocksGpout0 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio13CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio13CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio13CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio13CtrlFuncsel {
        Gpio13CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio13CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio13CtrlFuncsel) -> u8 {
        Gpio13CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio14CtrlFuncsel {
    Hstx2 = 0x0,
    Spi1Sclk = 0x01,
    Uart0Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA7 = 0x04,
    SiobProc14 = 0x05,
    Pio014 = 0x06,
    Pio114 = 0x07,
    Pio214 = 0x08,
    ClocksGpin1 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    Uart0Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio14CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio14CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio14CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio14CtrlFuncsel {
        Gpio14CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio14CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio14CtrlFuncsel) -> u8 {
        Gpio14CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio15CtrlFuncsel {
    Hstx3 = 0x0,
    Spi1Tx = 0x01,
    Uart0Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB7 = 0x04,
    SiobProc15 = 0x05,
    Pio015 = 0x06,
    Pio115 = 0x07,
    Pio215 = 0x08,
    ClocksGpout1 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    Uart0Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio15CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio15CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio15CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio15CtrlFuncsel {
        Gpio15CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio15CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio15CtrlFuncsel) -> u8 {
        Gpio15CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio16CtrlFuncsel {
    Hstx4 = 0x0,
    Spi0Rx = 0x01,
    Uart0Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA0 = 0x04,
    SiobProc16 = 0x05,
    Pio016 = 0x06,
    Pio116 = 0x07,
    Pio216 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio16CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio16CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio16CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio16CtrlFuncsel {
        Gpio16CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio16CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio16CtrlFuncsel) -> u8 {
        Gpio16CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio17CtrlFuncsel {
    Hstx5 = 0x0,
    Spi0SsN = 0x01,
    Uart0Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB0 = 0x04,
    SiobProc17 = 0x05,
    Pio017 = 0x06,
    Pio117 = 0x07,
    Pio217 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio17CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio17CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio17CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio17CtrlFuncsel {
        Gpio17CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio17CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio17CtrlFuncsel) -> u8 {
        Gpio17CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio18CtrlFuncsel {
    Hstx6 = 0x0,
    Spi0Sclk = 0x01,
    Uart0Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA1 = 0x04,
    SiobProc18 = 0x05,
    Pio018 = 0x06,
    Pio118 = 0x07,
    Pio218 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    Uart0Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio18CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio18CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio18CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio18CtrlFuncsel {
        Gpio18CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio18CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio18CtrlFuncsel) -> u8 {
        Gpio18CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio19CtrlFuncsel {
    Hstx7 = 0x0,
    Spi0Tx = 0x01,
    Uart0Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB1 = 0x04,
    SiobProc19 = 0x05,
    Pio019 = 0x06,
    Pio119 = 0x07,
    Pio219 = 0x08,
    XipSsN1 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    Uart0Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio19CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio19CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio19CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio19CtrlFuncsel {
        Gpio19CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio19CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio19CtrlFuncsel) -> u8 {
        Gpio19CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1CtrlFuncsel {
    JtagTms = 0x0,
    Spi0SsN = 0x01,
    Uart0Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB0 = 0x04,
    SiobProc1 = 0x05,
    Pio01 = 0x06,
    Pio11 = 0x07,
    Pio21 = 0x08,
    CoresightTraceclk = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio1CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio1CtrlFuncsel {
        Gpio1CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio1CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio1CtrlFuncsel) -> u8 {
        Gpio1CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio20CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Rx = 0x01,
    Uart1Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA2 = 0x04,
    SiobProc20 = 0x05,
    Pio020 = 0x06,
    Pio120 = 0x07,
    Pio220 = 0x08,
    ClocksGpin0 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio20CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio20CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio20CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio20CtrlFuncsel {
        Gpio20CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio20CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio20CtrlFuncsel) -> u8 {
        Gpio20CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio21CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0SsN = 0x01,
    Uart1Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB2 = 0x04,
    SiobProc21 = 0x05,
    Pio021 = 0x06,
    Pio121 = 0x07,
    Pio221 = 0x08,
    ClocksGpout0 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio21CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio21CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio21CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio21CtrlFuncsel {
        Gpio21CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio21CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio21CtrlFuncsel) -> u8 {
        Gpio21CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio22CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Sclk = 0x01,
    Uart1Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA3 = 0x04,
    SiobProc22 = 0x05,
    Pio022 = 0x06,
    Pio122 = 0x07,
    Pio222 = 0x08,
    ClocksGpin1 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    Uart1Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio22CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio22CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio22CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio22CtrlFuncsel {
        Gpio22CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio22CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio22CtrlFuncsel) -> u8 {
        Gpio22CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio23CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Tx = 0x01,
    Uart1Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB3 = 0x04,
    SiobProc23 = 0x05,
    Pio023 = 0x06,
    Pio123 = 0x07,
    Pio223 = 0x08,
    ClocksGpout1 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    Uart1Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio23CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio23CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio23CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio23CtrlFuncsel {
        Gpio23CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio23CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio23CtrlFuncsel) -> u8 {
        Gpio23CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio24CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Rx = 0x01,
    Uart1Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA4 = 0x04,
    SiobProc24 = 0x05,
    Pio024 = 0x06,
    Pio124 = 0x07,
    Pio224 = 0x08,
    ClocksGpout2 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio24CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio24CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio24CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio24CtrlFuncsel {
        Gpio24CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio24CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio24CtrlFuncsel) -> u8 {
        Gpio24CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio25CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1SsN = 0x01,
    Uart1Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB4 = 0x04,
    SiobProc25 = 0x05,
    Pio025 = 0x06,
    Pio125 = 0x07,
    Pio225 = 0x08,
    ClocksGpout3 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio25CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio25CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio25CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio25CtrlFuncsel {
        Gpio25CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio25CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio25CtrlFuncsel) -> u8 {
        Gpio25CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio26CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Sclk = 0x01,
    Uart1Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA5 = 0x04,
    SiobProc26 = 0x05,
    Pio026 = 0x06,
    Pio126 = 0x07,
    Pio226 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    Uart1Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio26CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio26CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio26CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio26CtrlFuncsel {
        Gpio26CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio26CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio26CtrlFuncsel) -> u8 {
        Gpio26CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio27CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Tx = 0x01,
    Uart1Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB5 = 0x04,
    SiobProc27 = 0x05,
    Pio027 = 0x06,
    Pio127 = 0x07,
    Pio227 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    Uart1Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio27CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio27CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio27CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio27CtrlFuncsel {
        Gpio27CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio27CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio27CtrlFuncsel) -> u8 {
        Gpio27CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio28CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Rx = 0x01,
    Uart0Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA6 = 0x04,
    SiobProc28 = 0x05,
    Pio028 = 0x06,
    Pio128 = 0x07,
    Pio228 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio28CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio28CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio28CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio28CtrlFuncsel {
        Gpio28CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio28CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio28CtrlFuncsel) -> u8 {
        Gpio28CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio29CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1SsN = 0x01,
    Uart0Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB6 = 0x04,
    SiobProc29 = 0x05,
    Pio029 = 0x06,
    Pio129 = 0x07,
    Pio229 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio29CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio29CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio29CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio29CtrlFuncsel {
        Gpio29CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio29CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio29CtrlFuncsel) -> u8 {
        Gpio29CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2CtrlFuncsel {
    JtagTdi = 0x0,
    Spi0Sclk = 0x01,
    Uart0Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA1 = 0x04,
    SiobProc2 = 0x05,
    Pio02 = 0x06,
    Pio12 = 0x07,
    Pio22 = 0x08,
    CoresightTracedata0 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    Uart0Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio2CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio2CtrlFuncsel {
        Gpio2CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio2CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio2CtrlFuncsel) -> u8 {
        Gpio2CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio30CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Sclk = 0x01,
    Uart0Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA7 = 0x04,
    SiobProc30 = 0x05,
    Pio030 = 0x06,
    Pio130 = 0x07,
    Pio230 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    Uart0Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio30CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio30CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio30CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio30CtrlFuncsel {
        Gpio30CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio30CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio30CtrlFuncsel) -> u8 {
        Gpio30CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio31CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Tx = 0x01,
    Uart0Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB7 = 0x04,
    SiobProc31 = 0x05,
    Pio031 = 0x06,
    Pio131 = 0x07,
    Pio231 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    Uart0Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio31CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio31CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio31CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio31CtrlFuncsel {
        Gpio31CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio31CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio31CtrlFuncsel) -> u8 {
        Gpio31CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio32CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Rx = 0x01,
    Uart0Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA8 = 0x04,
    SiobProc32 = 0x05,
    Pio032 = 0x06,
    Pio132 = 0x07,
    Pio232 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio32CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio32CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio32CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio32CtrlFuncsel {
        Gpio32CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio32CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio32CtrlFuncsel) -> u8 {
        Gpio32CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio33CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0SsN = 0x01,
    Uart0Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB8 = 0x04,
    SiobProc33 = 0x05,
    Pio033 = 0x06,
    Pio133 = 0x07,
    Pio233 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio33CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio33CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio33CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio33CtrlFuncsel {
        Gpio33CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio33CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio33CtrlFuncsel) -> u8 {
        Gpio33CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio34CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Sclk = 0x01,
    Uart0Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA9 = 0x04,
    SiobProc34 = 0x05,
    Pio034 = 0x06,
    Pio134 = 0x07,
    Pio234 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    Uart0Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio34CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio34CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio34CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio34CtrlFuncsel {
        Gpio34CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio34CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio34CtrlFuncsel) -> u8 {
        Gpio34CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio35CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Tx = 0x01,
    Uart0Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB9 = 0x04,
    SiobProc35 = 0x05,
    Pio035 = 0x06,
    Pio135 = 0x07,
    Pio235 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    Uart0Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio35CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio35CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio35CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio35CtrlFuncsel {
        Gpio35CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio35CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio35CtrlFuncsel) -> u8 {
        Gpio35CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio36CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Rx = 0x01,
    Uart1Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA10 = 0x04,
    SiobProc36 = 0x05,
    Pio036 = 0x06,
    Pio136 = 0x07,
    Pio236 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio36CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio36CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio36CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio36CtrlFuncsel {
        Gpio36CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio36CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio36CtrlFuncsel) -> u8 {
        Gpio36CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio37CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0SsN = 0x01,
    Uart1Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB10 = 0x04,
    SiobProc37 = 0x05,
    Pio037 = 0x06,
    Pio137 = 0x07,
    Pio237 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio37CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio37CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio37CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio37CtrlFuncsel {
        Gpio37CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio37CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio37CtrlFuncsel) -> u8 {
        Gpio37CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio38CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Sclk = 0x01,
    Uart1Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA11 = 0x04,
    SiobProc38 = 0x05,
    Pio038 = 0x06,
    Pio138 = 0x07,
    Pio238 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    Uart1Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio38CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio38CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio38CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio38CtrlFuncsel {
        Gpio38CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio38CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio38CtrlFuncsel) -> u8 {
        Gpio38CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio39CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Tx = 0x01,
    Uart1Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB11 = 0x04,
    SiobProc39 = 0x05,
    Pio039 = 0x06,
    Pio139 = 0x07,
    Pio239 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    Uart1Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio39CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio39CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio39CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio39CtrlFuncsel {
        Gpio39CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio39CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio39CtrlFuncsel) -> u8 {
        Gpio39CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3CtrlFuncsel {
    JtagTdo = 0x0,
    Spi0Tx = 0x01,
    Uart0Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB1 = 0x04,
    SiobProc3 = 0x05,
    Pio03 = 0x06,
    Pio13 = 0x07,
    Pio23 = 0x08,
    CoresightTracedata1 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    Uart0Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio3CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio3CtrlFuncsel {
        Gpio3CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio3CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio3CtrlFuncsel) -> u8 {
        Gpio3CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio40CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Rx = 0x01,
    Uart1Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA8 = 0x04,
    SiobProc40 = 0x05,
    Pio040 = 0x06,
    Pio140 = 0x07,
    Pio240 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio40CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio40CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio40CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio40CtrlFuncsel {
        Gpio40CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio40CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio40CtrlFuncsel) -> u8 {
        Gpio40CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio41CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1SsN = 0x01,
    Uart1Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB8 = 0x04,
    SiobProc41 = 0x05,
    Pio041 = 0x06,
    Pio141 = 0x07,
    Pio241 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio41CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio41CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio41CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio41CtrlFuncsel {
        Gpio41CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio41CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio41CtrlFuncsel) -> u8 {
        Gpio41CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio42CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Sclk = 0x01,
    Uart1Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA9 = 0x04,
    SiobProc42 = 0x05,
    Pio042 = 0x06,
    Pio142 = 0x07,
    Pio242 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    Uart1Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio42CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio42CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio42CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio42CtrlFuncsel {
        Gpio42CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio42CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio42CtrlFuncsel) -> u8 {
        Gpio42CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio43CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Tx = 0x01,
    Uart1Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB9 = 0x04,
    SiobProc43 = 0x05,
    Pio043 = 0x06,
    Pio143 = 0x07,
    Pio243 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    Uart1Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio43CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio43CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio43CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio43CtrlFuncsel {
        Gpio43CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio43CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio43CtrlFuncsel) -> u8 {
        Gpio43CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio44CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Rx = 0x01,
    Uart0Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA10 = 0x04,
    SiobProc44 = 0x05,
    Pio044 = 0x06,
    Pio144 = 0x07,
    Pio244 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio44CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio44CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio44CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio44CtrlFuncsel {
        Gpio44CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio44CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio44CtrlFuncsel) -> u8 {
        Gpio44CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio45CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1SsN = 0x01,
    Uart0Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB10 = 0x04,
    SiobProc45 = 0x05,
    Pio045 = 0x06,
    Pio145 = 0x07,
    Pio245 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio45CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio45CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio45CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio45CtrlFuncsel {
        Gpio45CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio45CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio45CtrlFuncsel) -> u8 {
        Gpio45CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio46CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Sclk = 0x01,
    Uart0Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA11 = 0x04,
    SiobProc46 = 0x05,
    Pio046 = 0x06,
    Pio146 = 0x07,
    Pio246 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    Uart0Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio46CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio46CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio46CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio46CtrlFuncsel {
        Gpio46CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio46CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio46CtrlFuncsel) -> u8 {
        Gpio46CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio47CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Tx = 0x01,
    Uart0Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB11 = 0x04,
    SiobProc47 = 0x05,
    Pio047 = 0x06,
    Pio147 = 0x07,
    Pio247 = 0x08,
    XipSsN1 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    Uart0Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio47CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio47CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio47CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio47CtrlFuncsel {
        Gpio47CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio47CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio47CtrlFuncsel) -> u8 {
        Gpio47CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Rx = 0x01,
    Uart1Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA2 = 0x04,
    SiobProc4 = 0x05,
    Pio04 = 0x06,
    Pio14 = 0x07,
    Pio24 = 0x08,
    CoresightTracedata2 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio4CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio4CtrlFuncsel {
        Gpio4CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio4CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio4CtrlFuncsel) -> u8 {
        Gpio4CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0SsN = 0x01,
    Uart1Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB2 = 0x04,
    SiobProc5 = 0x05,
    Pio05 = 0x06,
    Pio15 = 0x07,
    Pio25 = 0x08,
    CoresightTracedata3 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio5CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio5CtrlFuncsel {
        Gpio5CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio5CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio5CtrlFuncsel) -> u8 {
        Gpio5CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio6CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Sclk = 0x01,
    Uart1Cts = 0x02,
    I2c1Sda = 0x03,
    PwmA3 = 0x04,
    SiobProc6 = 0x05,
    Pio06 = 0x06,
    Pio16 = 0x07,
    Pio26 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    Uart1Tx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio6CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio6CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio6CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio6CtrlFuncsel {
        Gpio6CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio6CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio6CtrlFuncsel) -> u8 {
        Gpio6CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio7CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi0Tx = 0x01,
    Uart1Rts = 0x02,
    I2c1Scl = 0x03,
    PwmB3 = 0x04,
    SiobProc7 = 0x05,
    Pio07 = 0x06,
    Pio17 = 0x07,
    Pio27 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingVbusDetect = 0x0a,
    Uart1Rx = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio7CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio7CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio7CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio7CtrlFuncsel {
        Gpio7CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio7CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio7CtrlFuncsel) -> u8 {
        Gpio7CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio8CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1Rx = 0x01,
    Uart1Tx = 0x02,
    I2c0Sda = 0x03,
    PwmA4 = 0x04,
    SiobProc8 = 0x05,
    Pio08 = 0x06,
    Pio18 = 0x07,
    Pio28 = 0x08,
    XipSsN1 = 0x09,
    UsbMuxingVbusEn = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio8CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio8CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio8CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio8CtrlFuncsel {
        Gpio8CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio8CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio8CtrlFuncsel) -> u8 {
        Gpio8CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio9CtrlFuncsel {
    _RESERVED_0 = 0x0,
    Spi1SsN = 0x01,
    Uart1Rx = 0x02,
    I2c0Scl = 0x03,
    PwmB4 = 0x04,
    SiobProc9 = 0x05,
    Pio09 = 0x06,
    Pio19 = 0x07,
    Pio29 = 0x08,
    _RESERVED_9 = 0x09,
    UsbMuxingOvercurrDetect = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    Null = 0x1f,
}
impl Gpio9CtrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio9CtrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio9CtrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio9CtrlFuncsel {
        Gpio9CtrlFuncsel::from_bits(val)
    }
}
impl From<Gpio9CtrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio9CtrlFuncsel) -> u8 {
        Gpio9CtrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inover {
    #[doc = "don't invert the peri input."]
    Normal = 0x0,
    #[doc = "invert the peri input."]
    Invert = 0x01,
    #[doc = "drive peri input low."]
    Low = 0x02,
    #[doc = "drive peri input high."]
    High = 0x03,
}
impl Inover {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inover {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inover {
    #[inline(always)]
    fn from(val: u8) -> Inover {
        Inover::from_bits(val)
    }
}
impl From<Inover> for u8 {
    #[inline(always)]
    fn from(val: Inover) -> u8 {
        Inover::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqover {
    #[doc = "don't invert the interrupt."]
    Normal = 0x0,
    #[doc = "invert the interrupt."]
    Invert = 0x01,
    #[doc = "drive interrupt low."]
    Low = 0x02,
    #[doc = "drive interrupt high."]
    High = 0x03,
}
impl Irqover {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqover {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqover {
    #[inline(always)]
    fn from(val: u8) -> Irqover {
        Irqover::from_bits(val)
    }
}
impl From<Irqover> for u8 {
    #[inline(always)]
    fn from(val: Irqover) -> u8 {
        Irqover::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oeover {
    #[doc = "drive output enable from peripheral signal selected by funcsel."]
    Normal = 0x0,
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel."]
    Invert = 0x01,
    #[doc = "disable output."]
    Disable = 0x02,
    #[doc = "enable output."]
    Enable = 0x03,
}
impl Oeover {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oeover {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oeover {
    #[inline(always)]
    fn from(val: u8) -> Oeover {
        Oeover::from_bits(val)
    }
}
impl From<Oeover> for u8 {
    #[inline(always)]
    fn from(val: Oeover) -> u8 {
        Oeover::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outover {
    #[doc = "drive output from peripheral signal selected by funcsel."]
    Normal = 0x0,
    #[doc = "drive output from inverse of peripheral signal selected by funcsel."]
    Invert = 0x01,
    #[doc = "drive output low."]
    Low = 0x02,
    #[doc = "drive output high."]
    High = 0x03,
}
impl Outover {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outover {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outover {
    #[inline(always)]
    fn from(val: u8) -> Outover {
        Outover::from_bits(val)
    }
}
impl From<Outover> for u8 {
    #[inline(always)]
    fn from(val: Outover) -> u8 {
        Outover::to_bits(val)
    }
}
