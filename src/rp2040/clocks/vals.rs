#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkAdcCtrlAuxsrc {
    CLKSRC_PLL_USB = 0x0,
    CLKSRC_PLL_SYS = 0x01,
    ROSC_CLKSRC_PH = 0x02,
    XOSC_CLKSRC = 0x03,
    CLKSRC_GPIN0 = 0x04,
    CLKSRC_GPIN1 = 0x05,
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
    CLKSRC_PLL_SYS = 0x0,
    CLKSRC_GPIN0 = 0x01,
    CLKSRC_GPIN1 = 0x02,
    CLKSRC_PLL_USB = 0x03,
    ROSC_CLKSRC = 0x04,
    XOSC_CLKSRC = 0x05,
    CLK_SYS = 0x06,
    CLK_USB = 0x07,
    CLK_ADC = 0x08,
    CLK_RTC = 0x09,
    CLK_REF = 0x0a,
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
    CLK_SYS = 0x0,
    CLKSRC_PLL_SYS = 0x01,
    CLKSRC_PLL_USB = 0x02,
    ROSC_CLKSRC_PH = 0x03,
    XOSC_CLKSRC = 0x04,
    CLKSRC_GPIN0 = 0x05,
    CLKSRC_GPIN1 = 0x06,
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
    CLKSRC_PLL_USB = 0x0,
    CLKSRC_GPIN0 = 0x01,
    CLKSRC_GPIN1 = 0x02,
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
    ROSC_CLKSRC_PH = 0x0,
    CLKSRC_CLK_REF_AUX = 0x01,
    XOSC_CLKSRC = 0x02,
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
    CLKSRC_PLL_USB = 0x0,
    CLKSRC_PLL_SYS = 0x01,
    ROSC_CLKSRC_PH = 0x02,
    XOSC_CLKSRC = 0x03,
    CLKSRC_GPIN0 = 0x04,
    CLKSRC_GPIN1 = 0x05,
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
    CLKSRC_PLL_SYS = 0x0,
    CLKSRC_PLL_USB = 0x01,
    ROSC_CLKSRC = 0x02,
    XOSC_CLKSRC = 0x03,
    CLKSRC_GPIN0 = 0x04,
    CLKSRC_GPIN1 = 0x05,
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
    CLK_REF = 0x0,
    CLKSRC_CLK_SYS_AUX = 0x01,
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
    CLKSRC_PLL_USB = 0x0,
    CLKSRC_PLL_SYS = 0x01,
    ROSC_CLKSRC_PH = 0x02,
    XOSC_CLKSRC = 0x03,
    CLKSRC_GPIN0 = 0x04,
    CLKSRC_GPIN1 = 0x05,
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
pub struct Fc0src(u8);
impl Fc0src {
    pub const NULL: Self = Self(0x0);
    pub const PLL_SYS_CLKSRC_PRIMARY: Self = Self(0x01);
    pub const PLL_USB_CLKSRC_PRIMARY: Self = Self(0x02);
    pub const ROSC_CLKSRC: Self = Self(0x03);
    pub const ROSC_CLKSRC_PH: Self = Self(0x04);
    pub const XOSC_CLKSRC: Self = Self(0x05);
    pub const CLKSRC_GPIN0: Self = Self(0x06);
    pub const CLKSRC_GPIN1: Self = Self(0x07);
    pub const CLK_REF: Self = Self(0x08);
    pub const CLK_SYS: Self = Self(0x09);
    pub const CLK_PERI: Self = Self(0x0a);
    pub const CLK_USB: Self = Self(0x0b);
    pub const CLK_ADC: Self = Self(0x0c);
    pub const CLK_RTC: Self = Self(0x0d);
}
impl Fc0src {
    pub const fn from_bits(val: u8) -> Fc0src {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Fc0src {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NULL"),
            0x01 => f.write_str("PLL_SYS_CLKSRC_PRIMARY"),
            0x02 => f.write_str("PLL_USB_CLKSRC_PRIMARY"),
            0x03 => f.write_str("ROSC_CLKSRC"),
            0x04 => f.write_str("ROSC_CLKSRC_PH"),
            0x05 => f.write_str("XOSC_CLKSRC"),
            0x06 => f.write_str("CLKSRC_GPIN0"),
            0x07 => f.write_str("CLKSRC_GPIN1"),
            0x08 => f.write_str("CLK_REF"),
            0x09 => f.write_str("CLK_SYS"),
            0x0a => f.write_str("CLK_PERI"),
            0x0b => f.write_str("CLK_USB"),
            0x0c => f.write_str("CLK_ADC"),
            0x0d => f.write_str("CLK_RTC"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc0src {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NULL"),
            0x01 => defmt::write!(f, "PLL_SYS_CLKSRC_PRIMARY"),
            0x02 => defmt::write!(f, "PLL_USB_CLKSRC_PRIMARY"),
            0x03 => defmt::write!(f, "ROSC_CLKSRC"),
            0x04 => defmt::write!(f, "ROSC_CLKSRC_PH"),
            0x05 => defmt::write!(f, "XOSC_CLKSRC"),
            0x06 => defmt::write!(f, "CLKSRC_GPIN0"),
            0x07 => defmt::write!(f, "CLKSRC_GPIN1"),
            0x08 => defmt::write!(f, "CLK_REF"),
            0x09 => defmt::write!(f, "CLK_SYS"),
            0x0a => defmt::write!(f, "CLK_PERI"),
            0x0b => defmt::write!(f, "CLK_USB"),
            0x0c => defmt::write!(f, "CLK_ADC"),
            0x0d => defmt::write!(f, "CLK_RTC"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Fc0src {
    #[inline(always)]
    fn from(val: u8) -> Fc0src {
        Fc0src::from_bits(val)
    }
}
impl From<Fc0src> for u8 {
    #[inline(always)]
    fn from(val: Fc0src) -> u8 {
        Fc0src::to_bits(val)
    }
}
