#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Div(u16);
impl Div {
    pub const Pass: Self = Self(0x0aa0);
}
impl Div {
    pub const fn from_bits(val: u16) -> Div {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Div {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0aa0 => f.write_str("Pass"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Div {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0aa0 => defmt::write!(f, "Pass"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Div {
    #[inline(always)]
    fn from(val: u16) -> Div {
        Div::from_bits(val)
    }
}
impl From<Div> for u16 {
    #[inline(always)]
    fn from(val: Div) -> u16 {
        Div::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dormant(u32);
impl Dormant {
    pub const Dormant: Self = Self(0x636f_6d61);
    pub const Wake: Self = Self(0x7761_6b65);
}
impl Dormant {
    pub const fn from_bits(val: u32) -> Dormant {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Dormant {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x636f_6d61 => f.write_str("Dormant"),
            0x7761_6b65 => f.write_str("Wake"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dormant {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x636f_6d61 => defmt::write!(f, "Dormant"),
            0x7761_6b65 => defmt::write!(f, "Wake"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Dormant {
    #[inline(always)]
    fn from(val: u32) -> Dormant {
        Dormant::from_bits(val)
    }
}
impl From<Dormant> for u32 {
    #[inline(always)]
    fn from(val: Dormant) -> u32 {
        Dormant::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(u16);
impl Enable {
    pub const Disable: Self = Self(0x0d1e);
    pub const Enable: Self = Self(0x0fab);
}
impl Enable {
    pub const fn from_bits(val: u16) -> Enable {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Enable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0d1e => f.write_str("Disable"),
            0x0fab => f.write_str("Enable"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enable {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0d1e => defmt::write!(f, "Disable"),
            0x0fab => defmt::write!(f, "Enable"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Enable {
    #[inline(always)]
    fn from(val: u16) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u16 {
    #[inline(always)]
    fn from(val: Enable) -> u16 {
        Enable::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FreqRange(u16);
impl FreqRange {
    pub const Low: Self = Self(0x0fa4);
    pub const Medium: Self = Self(0x0fa5);
    pub const Toohigh: Self = Self(0x0fa6);
    pub const High: Self = Self(0x0fa7);
}
impl FreqRange {
    pub const fn from_bits(val: u16) -> FreqRange {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for FreqRange {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0fa4 => f.write_str("Low"),
            0x0fa5 => f.write_str("Medium"),
            0x0fa6 => f.write_str("Toohigh"),
            0x0fa7 => f.write_str("High"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqRange {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0fa4 => defmt::write!(f, "Low"),
            0x0fa5 => defmt::write!(f, "Medium"),
            0x0fa6 => defmt::write!(f, "Toohigh"),
            0x0fa7 => defmt::write!(f, "High"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for FreqRange {
    #[inline(always)]
    fn from(val: u16) -> FreqRange {
        FreqRange::from_bits(val)
    }
}
impl From<FreqRange> for u16 {
    #[inline(always)]
    fn from(val: FreqRange) -> u16 {
        FreqRange::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Passwd(u16);
impl Passwd {
    pub const Pass: Self = Self(0x9696);
}
impl Passwd {
    pub const fn from_bits(val: u16) -> Passwd {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Passwd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x9696 => f.write_str("Pass"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Passwd {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x9696 => defmt::write!(f, "Pass"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Passwd {
    #[inline(always)]
    fn from(val: u16) -> Passwd {
        Passwd::from_bits(val)
    }
}
impl From<Passwd> for u16 {
    #[inline(always)]
    fn from(val: Passwd) -> u16 {
        Passwd::to_bits(val)
    }
}
