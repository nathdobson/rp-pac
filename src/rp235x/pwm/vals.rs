#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch10CsrDivmode {
    #[doc = "Free-running counting at rate dictated by fractional divider."]
    Div = 0x0,
    #[doc = "Fractional divider operation is gated by the PWM B pin."]
    Level = 0x01,
    #[doc = "Counter advances with each rising edge of the PWM B pin."]
    Rise = 0x02,
    #[doc = "Counter advances with each falling edge of the PWM B pin."]
    Fall = 0x03,
}
impl Ch10CsrDivmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch10CsrDivmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch10CsrDivmode {
    #[inline(always)]
    fn from(val: u8) -> Ch10CsrDivmode {
        Ch10CsrDivmode::from_bits(val)
    }
}
impl From<Ch10CsrDivmode> for u8 {
    #[inline(always)]
    fn from(val: Ch10CsrDivmode) -> u8 {
        Ch10CsrDivmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch11CsrDivmode {
    #[doc = "Free-running counting at rate dictated by fractional divider."]
    Div = 0x0,
    #[doc = "Fractional divider operation is gated by the PWM B pin."]
    Level = 0x01,
    #[doc = "Counter advances with each rising edge of the PWM B pin."]
    Rise = 0x02,
    #[doc = "Counter advances with each falling edge of the PWM B pin."]
    Fall = 0x03,
}
impl Ch11CsrDivmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch11CsrDivmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch11CsrDivmode {
    #[inline(always)]
    fn from(val: u8) -> Ch11CsrDivmode {
        Ch11CsrDivmode::from_bits(val)
    }
}
impl From<Ch11CsrDivmode> for u8 {
    #[inline(always)]
    fn from(val: Ch11CsrDivmode) -> u8 {
        Ch11CsrDivmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Divmode {
    #[doc = "Free-running counting at rate dictated by fractional divider."]
    Div = 0x0,
    #[doc = "Fractional divider operation is gated by the PWM B pin."]
    Level = 0x01,
    #[doc = "Counter advances with each rising edge of the PWM B pin."]
    Rise = 0x02,
    #[doc = "Counter advances with each falling edge of the PWM B pin."]
    Fall = 0x03,
}
impl Divmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Divmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Divmode {
    #[inline(always)]
    fn from(val: u8) -> Divmode {
        Divmode::from_bits(val)
    }
}
impl From<Divmode> for u8 {
    #[inline(always)]
    fn from(val: Divmode) -> u8 {
        Divmode::to_bits(val)
    }
}
