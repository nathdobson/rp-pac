#[doc = "Indicates the armed/disarmed status of each alarm. A write to the corresponding ALARMx register arms the alarm. Alarms automatically disarm upon firing, but writing ones here will disarm immediately without waiting to fire."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Armed(pub u32);
impl Armed {
    #[must_use]
    #[inline(always)]
    pub const fn armed(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_armed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Armed {
    #[inline(always)]
    fn default() -> Armed {
        Armed(0)
    }
}
impl core::fmt::Debug for Armed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Armed")
            .field("armed", &self.armed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Armed {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Armed {{ armed: {=u8:?} }}", self.armed())
    }
}
#[doc = "Set bits high to enable pause when the corresponding debug ports are active"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgpause(pub u32);
impl Dbgpause {
    #[doc = "Pause when processor 0 is in debug mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pause when processor 0 is in debug mode"]
    #[inline(always)]
    pub const fn set_dbg0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pause when processor 1 is in debug mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Pause when processor 1 is in debug mode"]
    #[inline(always)]
    pub const fn set_dbg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Dbgpause {
    #[inline(always)]
    fn default() -> Dbgpause {
        Dbgpause(0)
    }
}
impl core::fmt::Debug for Dbgpause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgpause")
            .field("dbg0", &self.dbg0())
            .field("dbg1", &self.dbg1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgpause {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dbgpause {{ dbg0: {=bool:?}, dbg1: {=bool:?} }}",
            self.dbg0(),
            self.dbg1()
        )
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[must_use]
    #[inline(always)]
    pub const fn alarm(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_alarm(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
impl core::fmt::Debug for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int")
            .field("alarm[0]", &self.alarm(0usize))
            .field("alarm[1]", &self.alarm(1usize))
            .field("alarm[2]", &self.alarm(2usize))
            .field("alarm[3]", &self.alarm(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Int {{ alarm[0]: {=bool:?}, alarm[1]: {=bool:?}, alarm[2]: {=bool:?}, alarm[3]: {=bool:?} }}" , self . alarm (0usize) , self . alarm (1usize) , self . alarm (2usize) , self . alarm (3usize))
    }
}
#[doc = "Set locked bit to disable write access to timer Once set, cannot be cleared (without a reset)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Locked(pub u32);
impl Locked {
    #[must_use]
    #[inline(always)]
    pub const fn locked(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Locked {
    #[inline(always)]
    fn default() -> Locked {
        Locked(0)
    }
}
impl core::fmt::Debug for Locked {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Locked")
            .field("locked", &self.locked())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Locked {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Locked {{ locked: {=bool:?} }}", self.locked())
    }
}
#[doc = "Set high to pause the timer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pause(pub u32);
impl Pause {
    #[must_use]
    #[inline(always)]
    pub const fn pause(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pause(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Pause {
    #[inline(always)]
    fn default() -> Pause {
        Pause(0)
    }
}
impl core::fmt::Debug for Pause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pause")
            .field("pause", &self.pause())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pause {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pause {{ pause: {=bool:?} }}", self.pause())
    }
}
#[doc = "Selects the source for the timer. Defaults to the normal tick configured in the ticks block (typically configured to 1 microsecond). Writing to 1 will ignore the tick and count clk_sys cycles instead."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Source(pub u32);
impl Source {
    #[must_use]
    #[inline(always)]
    pub const fn clk_sys(&self) -> super::vals::ClkSys {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ClkSys::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_clk_sys(&mut self, val: super::vals::ClkSys) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Source {
    #[inline(always)]
    fn default() -> Source {
        Source(0)
    }
}
impl core::fmt::Debug for Source {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Source")
            .field("clk_sys", &self.clk_sys())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Source {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Source {{ clk_sys: {:?} }}", self.clk_sys())
    }
}
