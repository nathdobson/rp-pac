#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cs0Size {
    None = 0x0,
    _8k = 0x01,
    _16k = 0x02,
    _32k = 0x03,
    _64k = 0x04,
    _128k = 0x05,
    _256k = 0x06,
    _512k = 0x07,
    _1m = 0x08,
    _2m = 0x09,
    _4m = 0x0a,
    _8m = 0x0b,
    _16m = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cs0Size {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cs0Size {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cs0Size {
    #[inline(always)]
    fn from(val: u8) -> Cs0Size {
        Cs0Size::from_bits(val)
    }
}
impl From<Cs0Size> for u8 {
    #[inline(always)]
    fn from(val: Cs0Size) -> u8 {
        Cs0Size::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cs1Size(u16);
impl Cs1Size {
    pub const None: Self = Self(0x0);
    pub const _8k: Self = Self(0x01);
    pub const _16k: Self = Self(0x02);
    pub const _32k: Self = Self(0x03);
    pub const _64k: Self = Self(0x04);
    pub const _128k: Self = Self(0x05);
    pub const _256k: Self = Self(0x06);
    pub const _512k: Self = Self(0x07);
    pub const _1m: Self = Self(0x08);
    pub const _2m: Self = Self(0x09);
    pub const _4m: Self = Self(0x0a);
    pub const _8m: Self = Self(0x0b);
    pub const _16m: Self = Self(0x0c);
}
impl Cs1Size {
    pub const fn from_bits(val: u16) -> Cs1Size {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Cs1Size {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("None"),
            0x01 => f.write_str("_8k"),
            0x02 => f.write_str("_16k"),
            0x03 => f.write_str("_32k"),
            0x04 => f.write_str("_64k"),
            0x05 => f.write_str("_128k"),
            0x06 => f.write_str("_256k"),
            0x07 => f.write_str("_512k"),
            0x08 => f.write_str("_1m"),
            0x09 => f.write_str("_2m"),
            0x0a => f.write_str("_4m"),
            0x0b => f.write_str("_8m"),
            0x0c => f.write_str("_16m"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cs1Size {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "None"),
            0x01 => defmt::write!(f, "_8k"),
            0x02 => defmt::write!(f, "_16k"),
            0x03 => defmt::write!(f, "_32k"),
            0x04 => defmt::write!(f, "_64k"),
            0x05 => defmt::write!(f, "_128k"),
            0x06 => defmt::write!(f, "_256k"),
            0x07 => defmt::write!(f, "_512k"),
            0x08 => defmt::write!(f, "_1m"),
            0x09 => defmt::write!(f, "_2m"),
            0x0a => defmt::write!(f, "_4m"),
            0x0b => defmt::write!(f, "_8m"),
            0x0c => defmt::write!(f, "_16m"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Cs1Size {
    #[inline(always)]
    fn from(val: u16) -> Cs1Size {
        Cs1Size::from_bits(val)
    }
}
impl From<Cs1Size> for u16 {
    #[inline(always)]
    fn from(val: Cs1Size) -> u16 {
        Cs1Size::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PageLock {
    #[doc = "Bootloader permits user reads and writes to this page."]
    ReadWrite = 0x0,
    #[doc = "Bootloader permits user reads of this page."]
    ReadOnly = 0x01,
    #[doc = "Do not use. Behaves the same as INACCESSIBLE."]
    Reserved = 0x02,
    #[doc = "Bootloader does not permit user access to this page."]
    Inaccessible = 0x03,
}
impl PageLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PageLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PageLock {
    #[inline(always)]
    fn from(val: u8) -> PageLock {
        PageLock::from_bits(val)
    }
}
impl From<PageLock> for u8 {
    #[inline(always)]
    fn from(val: PageLock) -> u8 {
        PageLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PageLockNoKeyState {
    ReadOnly = 0x0,
    Inaccessible = 0x01,
}
impl PageLockNoKeyState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PageLockNoKeyState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PageLockNoKeyState {
    #[inline(always)]
    fn from(val: u8) -> PageLockNoKeyState {
        PageLockNoKeyState::from_bits(val)
    }
}
impl From<PageLockNoKeyState> for u8 {
    #[inline(always)]
    fn from(val: PageLockNoKeyState) -> u8 {
        PageLockNoKeyState::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Range(u16);
impl Range {
    pub const _115mhz: Self = Self(0x0);
    pub const _1030mhz: Self = Self(0x01);
    pub const _2560mhz: Self = Self(0x02);
    pub const _40100mhz: Self = Self(0x03);
}
impl Range {
    pub const fn from_bits(val: u16) -> Range {
        Self(val & 0x03ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Range {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_115mhz"),
            0x01 => f.write_str("_1030mhz"),
            0x02 => f.write_str("_2560mhz"),
            0x03 => f.write_str("_40100mhz"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Range {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_115mhz"),
            0x01 => defmt::write!(f, "_1030mhz"),
            0x02 => defmt::write!(f, "_2560mhz"),
            0x03 => defmt::write!(f, "_40100mhz"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Range {
    #[inline(always)]
    fn from(val: u16) -> Range {
        Range::from_bits(val)
    }
}
impl From<Range> for u16 {
    #[inline(always)]
    fn from(val: Range) -> u16 {
        Range::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UsbWhiteLabelAddr(u32);
impl UsbWhiteLabelAddr {
    pub const IndexUsbDeviceVidValue: Self = Self(0x0);
    pub const IndexUsbDevicePidValue: Self = Self(0x01);
    pub const IndexUsbDeviceBcdDeviceValue: Self = Self(0x02);
    pub const IndexUsbDeviceLangIdValue: Self = Self(0x03);
    pub const IndexUsbDeviceManufacturerStrdef: Self = Self(0x04);
    pub const IndexUsbDeviceProductStrdef: Self = Self(0x05);
    pub const IndexUsbDeviceSerialNumberStrdef: Self = Self(0x06);
    pub const IndexUsbConfigAttributesMaxPowerValues: Self = Self(0x07);
    pub const IndexVolumeLabelStrdef: Self = Self(0x08);
    pub const IndexScsiInquiryVendorStrdef: Self = Self(0x09);
    pub const IndexScsiInquiryProductStrdef: Self = Self(0x0a);
    pub const IndexScsiInquiryVersionStrdef: Self = Self(0x0b);
    pub const IndexIndexHtmRedirectUrlStrdef: Self = Self(0x0c);
    pub const IndexIndexHtmRedirectNameStrdef: Self = Self(0x0d);
    pub const IndexInfoUf2TxtModelStrdef: Self = Self(0x0e);
    pub const IndexInfoUf2TxtBoardIdStrdef: Self = Self(0x0f);
}
impl UsbWhiteLabelAddr {
    pub const fn from_bits(val: u32) -> UsbWhiteLabelAddr {
        Self(val & 0x00ff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UsbWhiteLabelAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("IndexUsbDeviceVidValue"),
            0x01 => f.write_str("IndexUsbDevicePidValue"),
            0x02 => f.write_str("IndexUsbDeviceBcdDeviceValue"),
            0x03 => f.write_str("IndexUsbDeviceLangIdValue"),
            0x04 => f.write_str("IndexUsbDeviceManufacturerStrdef"),
            0x05 => f.write_str("IndexUsbDeviceProductStrdef"),
            0x06 => f.write_str("IndexUsbDeviceSerialNumberStrdef"),
            0x07 => f.write_str("IndexUsbConfigAttributesMaxPowerValues"),
            0x08 => f.write_str("IndexVolumeLabelStrdef"),
            0x09 => f.write_str("IndexScsiInquiryVendorStrdef"),
            0x0a => f.write_str("IndexScsiInquiryProductStrdef"),
            0x0b => f.write_str("IndexScsiInquiryVersionStrdef"),
            0x0c => f.write_str("IndexIndexHtmRedirectUrlStrdef"),
            0x0d => f.write_str("IndexIndexHtmRedirectNameStrdef"),
            0x0e => f.write_str("IndexInfoUf2TxtModelStrdef"),
            0x0f => f.write_str("IndexInfoUf2TxtBoardIdStrdef"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbWhiteLabelAddr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "IndexUsbDeviceVidValue"),
            0x01 => defmt::write!(f, "IndexUsbDevicePidValue"),
            0x02 => defmt::write!(f, "IndexUsbDeviceBcdDeviceValue"),
            0x03 => defmt::write!(f, "IndexUsbDeviceLangIdValue"),
            0x04 => defmt::write!(f, "IndexUsbDeviceManufacturerStrdef"),
            0x05 => defmt::write!(f, "IndexUsbDeviceProductStrdef"),
            0x06 => defmt::write!(f, "IndexUsbDeviceSerialNumberStrdef"),
            0x07 => defmt::write!(f, "IndexUsbConfigAttributesMaxPowerValues"),
            0x08 => defmt::write!(f, "IndexVolumeLabelStrdef"),
            0x09 => defmt::write!(f, "IndexScsiInquiryVendorStrdef"),
            0x0a => defmt::write!(f, "IndexScsiInquiryProductStrdef"),
            0x0b => defmt::write!(f, "IndexScsiInquiryVersionStrdef"),
            0x0c => defmt::write!(f, "IndexIndexHtmRedirectUrlStrdef"),
            0x0d => defmt::write!(f, "IndexIndexHtmRedirectNameStrdef"),
            0x0e => defmt::write!(f, "IndexInfoUf2TxtModelStrdef"),
            0x0f => defmt::write!(f, "IndexInfoUf2TxtBoardIdStrdef"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UsbWhiteLabelAddr {
    #[inline(always)]
    fn from(val: u32) -> UsbWhiteLabelAddr {
        UsbWhiteLabelAddr::from_bits(val)
    }
}
impl From<UsbWhiteLabelAddr> for u32 {
    #[inline(always)]
    fn from(val: UsbWhiteLabelAddr) -> u32 {
        UsbWhiteLabelAddr::to_bits(val)
    }
}
