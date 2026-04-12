#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi {
    ptr: *mut u8,
}
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register 0, SSPCR0 on page 3-4."]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control register 1, SSPCR1 on page 3-5."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Data register, SSPDR on page 3-6."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status register, SSPSR on page 3-7."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Clock prescale register, SSPCPSR on page 3-8."]
    #[inline(always)]
    pub const fn cpsr(self) -> crate::common::Reg<regs::cpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9."]
    #[inline(always)]
    pub const fn imsc(self) -> crate::common::Reg<regs::imsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Raw interrupt status register, SSPRIS on page 3-10."]
    #[inline(always)]
    pub const fn ris(self) -> crate::common::Reg<regs::ris, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Masked interrupt status register, SSPMIS on page 3-11."]
    #[inline(always)]
    pub const fn mis(self) -> crate::common::Reg<regs::mis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Interrupt clear register, SSPICR on page 3-11."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DMA control register, SSPDMACR on page 3-12."]
    #[inline(always)]
    pub const fn dmacr(self) -> crate::common::Reg<regs::dmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13."]
    #[inline(always)]
    pub const fn periphid0(self) -> crate::common::Reg<regs::periphid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13."]
    #[inline(always)]
    pub const fn periphid1(self) -> crate::common::Reg<regs::periphid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13."]
    #[inline(always)]
    pub const fn periphid2(self) -> crate::common::Reg<regs::periphid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13."]
    #[inline(always)]
    pub const fn periphid3(self) -> crate::common::Reg<regs::periphid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16."]
    #[inline(always)]
    pub const fn pcellid0(self) -> crate::common::Reg<regs::pcellid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16."]
    #[inline(always)]
    pub const fn pcellid1(self) -> crate::common::Reg<regs::pcellid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16."]
    #[inline(always)]
    pub const fn pcellid2(self) -> crate::common::Reg<regs::pcellid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16."]
    #[inline(always)]
    pub const fn pcellid3(self) -> crate::common::Reg<regs::pcellid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
