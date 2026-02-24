#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioCtrl(pub u32);
impl GpioCtrl {
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    #[must_use]
    #[inline(always)]
    pub const fn funcsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    #[inline(always)]
    pub const fn set_funcsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn outover(&self) -> super::vals::Outover {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Outover::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_outover(&mut self, val: super::vals::Outover) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn oeover(&self) -> super::vals::Oeover {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Oeover::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_oeover(&mut self, val: super::vals::Oeover) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn inover(&self) -> super::vals::Inover {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Inover::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_inover(&mut self, val: super::vals::Inover) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn irqover(&self) -> super::vals::Irqover {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Irqover::from_bits(val as u8)
    }
    #[inline(always)]
    pub const fn set_irqover(&mut self, val: super::vals::Irqover) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for GpioCtrl {
    #[inline(always)]
    fn default() -> GpioCtrl {
        GpioCtrl(0)
    }
}
impl core::fmt::Debug for GpioCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpioCtrl")
            .field("funcsel", &self.funcsel())
            .field("outover", &self.outover())
            .field("oeover", &self.oeover())
            .field("inover", &self.inover())
            .field("irqover", &self.irqover())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpioCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "GpioCtrl {{ funcsel: {=u8:?}, outover: {:?}, oeover: {:?}, inover: {:?}, irqover: {:?} }}" , self . funcsel () , self . outover () , self . oeover () , self . inover () , self . irqover ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioStatus(pub u32);
impl GpioStatus {
    #[doc = "output signal to pad after register override is applied"]
    #[must_use]
    #[inline(always)]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    #[inline(always)]
    pub const fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "output enable to pad after register override is applied"]
    #[must_use]
    #[inline(always)]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    #[inline(always)]
    pub const fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "input signal from pad, before filtering and override are applied"]
    #[must_use]
    #[inline(always)]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before filtering and override are applied"]
    #[inline(always)]
    pub const fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "interrupt to processors, after override is applied"]
    #[must_use]
    #[inline(always)]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    #[inline(always)]
    pub const fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for GpioStatus {
    #[inline(always)]
    fn default() -> GpioStatus {
        GpioStatus(0)
    }
}
impl core::fmt::Debug for GpioStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpioStatus")
            .field("outtopad", &self.outtopad())
            .field("oetopad", &self.oetopad())
            .field("infrompad", &self.infrompad())
            .field("irqtoproc", &self.irqtoproc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpioStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "GpioStatus {{ outtopad: {=bool:?}, oetopad: {=bool:?}, infrompad: {=bool:?}, irqtoproc: {=bool:?} }}" , self . outtopad () , self . oetopad () , self . infrompad () , self . irqtoproc ())
    }
}
#[doc = "Interrupt Enable for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[must_use]
    #[inline(always)]
    pub const fn level_low(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_level_low(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[must_use]
    #[inline(always)]
    pub const fn level_high(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 1usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_level_high(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 1usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edge_low(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 2usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_edge_low(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 2usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edge_high(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_edge_high(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
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
            .field("level_low[0]", &self.level_low(0usize))
            .field("level_low[1]", &self.level_low(1usize))
            .field("level_low[2]", &self.level_low(2usize))
            .field("level_low[3]", &self.level_low(3usize))
            .field("level_low[4]", &self.level_low(4usize))
            .field("level_low[5]", &self.level_low(5usize))
            .field("level_low[6]", &self.level_low(6usize))
            .field("level_low[7]", &self.level_low(7usize))
            .field("level_high[0]", &self.level_high(0usize))
            .field("level_high[1]", &self.level_high(1usize))
            .field("level_high[2]", &self.level_high(2usize))
            .field("level_high[3]", &self.level_high(3usize))
            .field("level_high[4]", &self.level_high(4usize))
            .field("level_high[5]", &self.level_high(5usize))
            .field("level_high[6]", &self.level_high(6usize))
            .field("level_high[7]", &self.level_high(7usize))
            .field("edge_low[0]", &self.edge_low(0usize))
            .field("edge_low[1]", &self.edge_low(1usize))
            .field("edge_low[2]", &self.edge_low(2usize))
            .field("edge_low[3]", &self.edge_low(3usize))
            .field("edge_low[4]", &self.edge_low(4usize))
            .field("edge_low[5]", &self.edge_low(5usize))
            .field("edge_low[6]", &self.edge_low(6usize))
            .field("edge_low[7]", &self.edge_low(7usize))
            .field("edge_high[0]", &self.edge_high(0usize))
            .field("edge_high[1]", &self.edge_high(1usize))
            .field("edge_high[2]", &self.edge_high(2usize))
            .field("edge_high[3]", &self.edge_high(3usize))
            .field("edge_high[4]", &self.edge_high(4usize))
            .field("edge_high[5]", &self.edge_high(5usize))
            .field("edge_high[6]", &self.edge_high(6usize))
            .field("edge_high[7]", &self.edge_high(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Int {{ level_low[0]: {=bool:?}, level_low[1]: {=bool:?}, level_low[2]: {=bool:?}, level_low[3]: {=bool:?}, level_low[4]: {=bool:?}, level_low[5]: {=bool:?}, level_low[6]: {=bool:?}, level_low[7]: {=bool:?}, level_high[0]: {=bool:?}, level_high[1]: {=bool:?}, level_high[2]: {=bool:?}, level_high[3]: {=bool:?}, level_high[4]: {=bool:?}, level_high[5]: {=bool:?}, level_high[6]: {=bool:?}, level_high[7]: {=bool:?}, edge_low[0]: {=bool:?}, edge_low[1]: {=bool:?}, edge_low[2]: {=bool:?}, edge_low[3]: {=bool:?}, edge_low[4]: {=bool:?}, edge_low[5]: {=bool:?}, edge_low[6]: {=bool:?}, edge_low[7]: {=bool:?}, edge_high[0]: {=bool:?}, edge_high[1]: {=bool:?}, edge_high[2]: {=bool:?}, edge_high[3]: {=bool:?}, edge_high[4]: {=bool:?}, edge_high[5]: {=bool:?}, edge_high[6]: {=bool:?}, edge_high[7]: {=bool:?} }}" , self . level_low (0usize) , self . level_low (1usize) , self . level_low (2usize) , self . level_low (3usize) , self . level_low (4usize) , self . level_low (5usize) , self . level_low (6usize) , self . level_low (7usize) , self . level_high (0usize) , self . level_high (1usize) , self . level_high (2usize) , self . level_high (3usize) , self . level_high (4usize) , self . level_high (5usize) , self . level_high (6usize) , self . level_high (7usize) , self . edge_low (0usize) , self . edge_low (1usize) , self . edge_low (2usize) , self . edge_low (3usize) , self . edge_low (4usize) , self . edge_low (5usize) , self . edge_low (6usize) , self . edge_low (7usize) , self . edge_high (0usize) , self . edge_high (1usize) , self . edge_high (2usize) , self . edge_high (3usize) , self . edge_high (4usize) , self . edge_high (5usize) , self . edge_high (6usize) , self . edge_high (7usize))
    }
}
