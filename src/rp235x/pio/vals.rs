#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExecctrlStatusN {
    #[doc = "Index 0-7 of an IRQ flag in this PIO block."]
    Irq = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Index 0-7 of an IRQ flag in the next lower-numbered PIO block."]
    IrqPrevpio = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Index 0-7 of an IRQ flag in the next higher-numbered PIO block."]
    IrqNextpio = 0x10,
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
    _RESERVED_1f = 0x1f,
}
impl ExecctrlStatusN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExecctrlStatusN {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExecctrlStatusN {
    #[inline(always)]
    fn from(val: u8) -> ExecctrlStatusN {
        ExecctrlStatusN::from_bits(val)
    }
}
impl From<ExecctrlStatusN> for u8 {
    #[inline(always)]
    fn from(val: ExecctrlStatusN) -> u8 {
        ExecctrlStatusN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExecctrlStatusSel {
    #[doc = "All-ones if TX FIFO level < N, otherwise all-zeroes."]
    Txlevel = 0x0,
    #[doc = "All-ones if RX FIFO level < N, otherwise all-zeroes."]
    Rxlevel = 0x01,
    #[doc = "All-ones if the indexed IRQ flag is raised, otherwise all-zeroes."]
    Irq = 0x02,
    _RESERVED_3 = 0x03,
}
impl ExecctrlStatusSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExecctrlStatusSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExecctrlStatusSel {
    #[inline(always)]
    fn from(val: u8) -> ExecctrlStatusSel {
        ExecctrlStatusSel::from_bits(val)
    }
}
impl From<ExecctrlStatusSel> for u8 {
    #[inline(always)]
    fn from(val: ExecctrlStatusSel) -> u8 {
        ExecctrlStatusSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Version {
    #[doc = "Version 0 (RP2040)."]
    V0 = 0x0,
    #[doc = "Version 1 (RP2350)."]
    V1 = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Version {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Version {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Version {
    #[inline(always)]
    fn from(val: u8) -> Version {
        Version::from_bits(val)
    }
}
impl From<Version> for u8 {
    #[inline(always)]
    fn from(val: Version) -> u8 {
        Version::to_bits(val)
    }
}
