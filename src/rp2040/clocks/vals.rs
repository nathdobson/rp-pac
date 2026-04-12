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
    RoscClksrc = 0x04,
    XoscClksrc = 0x05,
    ClkSys = 0x06,
    ClkUsb = 0x07,
    ClkAdc = 0x08,
    ClkRtc = 0x09,
    ClkRef = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
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
    _RESERVED_3 = 0x03,
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
    _RESERVED_3 = 0x03,
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
pub enum ClkRtcCtrlAuxsrc {
    ClksrcPllUsb = 0x0,
    ClksrcPllSys = 0x01,
    RoscClksrcPh = 0x02,
    XoscClksrc = 0x03,
    ClksrcGpin0 = 0x04,
    ClksrcGpin1 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkRtcCtrlAuxsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkRtcCtrlAuxsrc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkRtcCtrlAuxsrc {
    #[inline(always)]
    fn from(val: u8) -> ClkRtcCtrlAuxsrc {
        ClkRtcCtrlAuxsrc::from_bits(val)
    }
}
impl From<ClkRtcCtrlAuxsrc> for u8 {
    #[inline(always)]
    fn from(val: ClkRtcCtrlAuxsrc) -> u8 {
        ClkRtcCtrlAuxsrc::to_bits(val)
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
    pub const ClkRtc: Self = Self(0x0d);
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
            0x0d => f.write_str("ClkRtc"),
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
            0x0d => defmt::write!(f, "ClkRtc"),
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
