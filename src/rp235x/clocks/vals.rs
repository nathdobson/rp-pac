#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkAdcCtrlAuxsrc {
    ClksrcPllUsb = 0x0,
    ClksrcPllSys = 0x01,
    RoscClksrcPh = 0x02,
    XoscClksrc = 0x03,
    ClksrcGpin0 = 0x04,
    ClksrcGpin1 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkAdcCtrlAuxsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkAdcCtrlAuxsrc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkAdcCtrlAuxsrc {
    #[inline(always)]
    fn from(val: u8) -> ClkAdcCtrlAuxsrc {
        ClkAdcCtrlAuxsrc::from_bits(val)
    }
}
impl From<ClkAdcCtrlAuxsrc> for u8 {
    #[inline(always)]
    fn from(val: ClkAdcCtrlAuxsrc) -> u8 {
        ClkAdcCtrlAuxsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkGpoutCtrlAuxsrc {
    ClksrcPllSys = 0x0,
    ClksrcGpin0 = 0x01,
    ClksrcGpin1 = 0x02,
    ClksrcPllUsb = 0x03,
    ClksrcPllUsbPrimaryRefOpcg = 0x04,
    RoscClksrc = 0x05,
    XoscClksrc = 0x06,
    LposcClksrc = 0x07,
    ClkSys = 0x08,
    ClkUsb = 0x09,
    ClkAdc = 0x0a,
    ClkRef = 0x0b,
    ClkPeri = 0x0c,
    ClkHstx = 0x0d,
    OtpClk2fc = 0x0e,
    _RESERVED_f = 0x0f,
}
impl ClkGpoutCtrlAuxsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkGpoutCtrlAuxsrc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkGpoutCtrlAuxsrc {
    #[inline(always)]
    fn from(val: u8) -> ClkGpoutCtrlAuxsrc {
        ClkGpoutCtrlAuxsrc::from_bits(val)
    }
}
impl From<ClkGpoutCtrlAuxsrc> for u8 {
    #[inline(always)]
    fn from(val: ClkGpoutCtrlAuxsrc) -> u8 {
        ClkGpoutCtrlAuxsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkHstxCtrlAuxsrc {
    ClkSys = 0x0,
    ClksrcPllSys = 0x01,
    ClksrcPllUsb = 0x02,
    ClksrcGpin0 = 0x03,
    ClksrcGpin1 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkHstxCtrlAuxsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkHstxCtrlAuxsrc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkHstxCtrlAuxsrc {
    #[inline(always)]
    fn from(val: u8) -> ClkHstxCtrlAuxsrc {
        ClkHstxCtrlAuxsrc::from_bits(val)
    }
}
impl From<ClkHstxCtrlAuxsrc> for u8 {
    #[inline(always)]
    fn from(val: ClkHstxCtrlAuxsrc) -> u8 {
        ClkHstxCtrlAuxsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkPeriCtrlAuxsrc {
    ClkSys = 0x0,
    ClksrcPllSys = 0x01,
    ClksrcPllUsb = 0x02,
    RoscClksrcPh = 0x03,
    XoscClksrc = 0x04,
    ClksrcGpin0 = 0x05,
    ClksrcGpin1 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkPeriCtrlAuxsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkPeriCtrlAuxsrc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkPeriCtrlAuxsrc {
    #[inline(always)]
    fn from(val: u8) -> ClkPeriCtrlAuxsrc {
        ClkPeriCtrlAuxsrc::from_bits(val)
    }
}
impl From<ClkPeriCtrlAuxsrc> for u8 {
    #[inline(always)]
    fn from(val: ClkPeriCtrlAuxsrc) -> u8 {
        ClkPeriCtrlAuxsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkRefCtrlAuxsrc {
    ClksrcPllUsb = 0x0,
    ClksrcGpin0 = 0x01,
    ClksrcGpin1 = 0x02,
    ClksrcPllUsbPrimaryRefOpcg = 0x03,
}
impl ClkRefCtrlAuxsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkRefCtrlAuxsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkRefCtrlAuxsrc {
    #[inline(always)]
    fn from(val: u8) -> ClkRefCtrlAuxsrc {
        ClkRefCtrlAuxsrc::from_bits(val)
    }
}
impl From<ClkRefCtrlAuxsrc> for u8 {
    #[inline(always)]
    fn from(val: ClkRefCtrlAuxsrc) -> u8 {
        ClkRefCtrlAuxsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkRefCtrlSrc {
    RoscClksrcPh = 0x0,
    ClksrcClkRefAux = 0x01,
    XoscClksrc = 0x02,
    LposcClksrc = 0x03,
}
impl ClkRefCtrlSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkRefCtrlSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkRefCtrlSrc {
    #[inline(always)]
    fn from(val: u8) -> ClkRefCtrlSrc {
        ClkRefCtrlSrc::from_bits(val)
    }
}
impl From<ClkRefCtrlSrc> for u8 {
    #[inline(always)]
    fn from(val: ClkRefCtrlSrc) -> u8 {
        ClkRefCtrlSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSysCtrlAuxsrc {
    ClksrcPllSys = 0x0,
    ClksrcPllUsb = 0x01,
    RoscClksrc = 0x02,
    XoscClksrc = 0x03,
    ClksrcGpin0 = 0x04,
    ClksrcGpin1 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkSysCtrlAuxsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSysCtrlAuxsrc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSysCtrlAuxsrc {
    #[inline(always)]
    fn from(val: u8) -> ClkSysCtrlAuxsrc {
        ClkSysCtrlAuxsrc::from_bits(val)
    }
}
impl From<ClkSysCtrlAuxsrc> for u8 {
    #[inline(always)]
    fn from(val: ClkSysCtrlAuxsrc) -> u8 {
        ClkSysCtrlAuxsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSysCtrlSrc {
    ClkRef = 0x0,
    ClksrcClkSysAux = 0x01,
}
impl ClkSysCtrlSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSysCtrlSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSysCtrlSrc {
    #[inline(always)]
    fn from(val: u8) -> ClkSysCtrlSrc {
        ClkSysCtrlSrc::from_bits(val)
    }
}
impl From<ClkSysCtrlSrc> for u8 {
    #[inline(always)]
    fn from(val: ClkSysCtrlSrc) -> u8 {
        ClkSysCtrlSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkUsbCtrlAuxsrc {
    ClksrcPllUsb = 0x0,
    ClksrcPllSys = 0x01,
    RoscClksrcPh = 0x02,
    XoscClksrc = 0x03,
    ClksrcGpin0 = 0x04,
    ClksrcGpin1 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkUsbCtrlAuxsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkUsbCtrlAuxsrc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkUsbCtrlAuxsrc {
    #[inline(always)]
    fn from(val: u8) -> ClkUsbCtrlAuxsrc {
        ClkUsbCtrlAuxsrc::from_bits(val)
    }
}
impl From<ClkUsbCtrlAuxsrc> for u8 {
    #[inline(always)]
    fn from(val: ClkUsbCtrlAuxsrc) -> u8 {
        ClkUsbCtrlAuxsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftclkLposcCtrlSrc {
    Null = 0x0,
    ClksrcPllUsbPrimaryLposc = 0x01,
    ClksrcGpin1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl DftclkLposcCtrlSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftclkLposcCtrlSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftclkLposcCtrlSrc {
    #[inline(always)]
    fn from(val: u8) -> DftclkLposcCtrlSrc {
        DftclkLposcCtrlSrc::from_bits(val)
    }
}
impl From<DftclkLposcCtrlSrc> for u8 {
    #[inline(always)]
    fn from(val: DftclkLposcCtrlSrc) -> u8 {
        DftclkLposcCtrlSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftclkRoscCtrlSrc {
    Null = 0x0,
    ClksrcPllSysPrimaryRosc = 0x01,
    ClksrcGpin1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl DftclkRoscCtrlSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftclkRoscCtrlSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftclkRoscCtrlSrc {
    #[inline(always)]
    fn from(val: u8) -> DftclkRoscCtrlSrc {
        DftclkRoscCtrlSrc::from_bits(val)
    }
}
impl From<DftclkRoscCtrlSrc> for u8 {
    #[inline(always)]
    fn from(val: DftclkRoscCtrlSrc) -> u8 {
        DftclkRoscCtrlSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftclkXoscCtrlSrc {
    Null = 0x0,
    ClksrcPllUsbPrimary = 0x01,
    ClksrcGpin0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl DftclkXoscCtrlSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftclkXoscCtrlSrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftclkXoscCtrlSrc {
    #[inline(always)]
    fn from(val: u8) -> DftclkXoscCtrlSrc {
        DftclkXoscCtrlSrc::from_bits(val)
    }
}
impl From<DftclkXoscCtrlSrc> for u8 {
    #[inline(always)]
    fn from(val: DftclkXoscCtrlSrc) -> u8 {
        DftclkXoscCtrlSrc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fc0Src(u8);
impl Fc0Src {
    pub const Null: Self = Self(0x0);
    pub const PllSysClksrcPrimary: Self = Self(0x01);
    pub const PllUsbClksrcPrimary: Self = Self(0x02);
    pub const RoscClksrc: Self = Self(0x03);
    pub const RoscClksrcPh: Self = Self(0x04);
    pub const XoscClksrc: Self = Self(0x05);
    pub const ClksrcGpin0: Self = Self(0x06);
    pub const ClksrcGpin1: Self = Self(0x07);
    pub const ClkRef: Self = Self(0x08);
    pub const ClkSys: Self = Self(0x09);
    pub const ClkPeri: Self = Self(0x0a);
    pub const ClkUsb: Self = Self(0x0b);
    pub const ClkAdc: Self = Self(0x0c);
    pub const ClkHstx: Self = Self(0x0d);
    pub const LposcClksrc: Self = Self(0x0e);
    pub const OtpClk2fc: Self = Self(0x0f);
    pub const PllUsbClksrcPrimaryDft: Self = Self(0x10);
}
impl Fc0Src {
    pub const fn from_bits(val: u8) -> Fc0Src {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Fc0Src {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Null"),
            0x01 => f.write_str("PllSysClksrcPrimary"),
            0x02 => f.write_str("PllUsbClksrcPrimary"),
            0x03 => f.write_str("RoscClksrc"),
            0x04 => f.write_str("RoscClksrcPh"),
            0x05 => f.write_str("XoscClksrc"),
            0x06 => f.write_str("ClksrcGpin0"),
            0x07 => f.write_str("ClksrcGpin1"),
            0x08 => f.write_str("ClkRef"),
            0x09 => f.write_str("ClkSys"),
            0x0a => f.write_str("ClkPeri"),
            0x0b => f.write_str("ClkUsb"),
            0x0c => f.write_str("ClkAdc"),
            0x0d => f.write_str("ClkHstx"),
            0x0e => f.write_str("LposcClksrc"),
            0x0f => f.write_str("OtpClk2fc"),
            0x10 => f.write_str("PllUsbClksrcPrimaryDft"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc0Src {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Null"),
            0x01 => defmt::write!(f, "PllSysClksrcPrimary"),
            0x02 => defmt::write!(f, "PllUsbClksrcPrimary"),
            0x03 => defmt::write!(f, "RoscClksrc"),
            0x04 => defmt::write!(f, "RoscClksrcPh"),
            0x05 => defmt::write!(f, "XoscClksrc"),
            0x06 => defmt::write!(f, "ClksrcGpin0"),
            0x07 => defmt::write!(f, "ClksrcGpin1"),
            0x08 => defmt::write!(f, "ClkRef"),
            0x09 => defmt::write!(f, "ClkSys"),
            0x0a => defmt::write!(f, "ClkPeri"),
            0x0b => defmt::write!(f, "ClkUsb"),
            0x0c => defmt::write!(f, "ClkAdc"),
            0x0d => defmt::write!(f, "ClkHstx"),
            0x0e => defmt::write!(f, "LposcClksrc"),
            0x0f => defmt::write!(f, "OtpClk2fc"),
            0x10 => defmt::write!(f, "PllUsbClksrcPrimaryDft"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Fc0Src {
    #[inline(always)]
    fn from(val: u8) -> Fc0Src {
        Fc0Src::from_bits(val)
    }
}
impl From<Fc0Src> for u8 {
    #[inline(always)]
    fn from(val: Fc0Src) -> u8 {
        Fc0Src::to_bits(val)
    }
}
