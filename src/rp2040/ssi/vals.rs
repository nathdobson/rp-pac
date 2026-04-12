#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InstL {
    #[doc = "No instruction."]
    None = 0x0,
    #[doc = "4-bit instruction."]
    _4b = 0x01,
    #[doc = "8-bit instruction."]
    _8b = 0x02,
    #[doc = "16-bit instruction."]
    _16b = 0x03,
}
impl InstL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InstL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InstL {
    #[inline(always)]
    fn from(val: u8) -> InstL {
        InstL::from_bits(val)
    }
}
impl From<InstL> for u8 {
    #[inline(always)]
    fn from(val: InstL) -> u8 {
        InstL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiFrf {
    #[doc = "Standard 1-bit SPI frame format; 1 bit per SCK, full-duplex."]
    Std = 0x0,
    #[doc = "Dual-SPI frame format; two bits per SCK, half-duplex."]
    Dual = 0x01,
    #[doc = "Quad-SPI frame format; four bits per SCK, half-duplex."]
    Quad = 0x02,
    _RESERVED_3 = 0x03,
}
impl SpiFrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiFrf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiFrf {
    #[inline(always)]
    fn from(val: u8) -> SpiFrf {
        SpiFrf::from_bits(val)
    }
}
impl From<SpiFrf> for u8 {
    #[inline(always)]
    fn from(val: SpiFrf) -> u8 {
        SpiFrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmod {
    #[doc = "Both transmit and receive."]
    TxAndRx = 0x0,
    #[doc = "Transmit only (not for FRF == 0, standard SPI mode)."]
    TxOnly = 0x01,
    #[doc = "Receive only (not for FRF == 0, standard SPI mode)."]
    RxOnly = 0x02,
    #[doc = "EEPROM read mode (TX then RX; RX starts after control data TX'd)."]
    EepromRead = 0x03,
}
impl Tmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmod {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmod {
    #[inline(always)]
    fn from(val: u8) -> Tmod {
        Tmod::from_bits(val)
    }
}
impl From<Tmod> for u8 {
    #[inline(always)]
    fn from(val: Tmod) -> u8 {
        Tmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TransType {
    #[doc = "Command and address both in standard SPI frame format."]
    _1c1a = 0x0,
    #[doc = "Command in standard SPI format, address in format specified by FRF."]
    _1c2a = 0x01,
    #[doc = "Command and address both in format specified by FRF (e.g. Dual-SPI)."]
    _2c2a = 0x02,
    _RESERVED_3 = 0x03,
}
impl TransType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TransType {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TransType {
    #[inline(always)]
    fn from(val: u8) -> TransType {
        TransType::from_bits(val)
    }
}
impl From<TransType> for u8 {
    #[inline(always)]
    fn from(val: TransType) -> u8 {
        TransType::to_bits(val)
    }
}
