#[doc = "Clock prescale register, SSPCPSR on page 3-8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpsr(pub u32);
impl Cpsr {
    #[doc = "Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    #[must_use]
    #[inline(always)]
    pub const fn cpsdvsr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    #[inline(always)]
    pub const fn set_cpsdvsr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cpsr {
    #[inline(always)]
    fn default() -> Cpsr {
        Cpsr(0)
    }
}
impl core::fmt::Debug for Cpsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpsr")
            .field("cpsdvsr", &self.cpsdvsr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cpsr {{ cpsdvsr: {=u8:?} }}", self.cpsdvsr())
    }
}
#[doc = "Control register 0, SSPCR0 on page 3-4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr0(pub u32);
impl Cr0 {
    #[doc = "Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
    #[must_use]
    #[inline(always)]
    pub const fn dss(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
    #[inline(always)]
    pub const fn set_dss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation."]
    #[must_use]
    #[inline(always)]
    pub const fn frf(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation."]
    #[inline(always)]
    pub const fn set_frf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    #[must_use]
    #[inline(always)]
    pub const fn spo(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    #[inline(always)]
    pub const fn set_spo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    #[must_use]
    #[inline(always)]
    pub const fn sph(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    #[inline(always)]
    pub const fn set_sph(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
    #[must_use]
    #[inline(always)]
    pub const fn scr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
    #[inline(always)]
    pub const fn set_scr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Cr0 {
    #[inline(always)]
    fn default() -> Cr0 {
        Cr0(0)
    }
}
impl core::fmt::Debug for Cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr0")
            .field("dss", &self.dss())
            .field("frf", &self.frf())
            .field("spo", &self.spo())
            .field("sph", &self.sph())
            .field("scr", &self.scr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr0 {{ dss: {=u8:?}, frf: {=u8:?}, spo: {=bool:?}, sph: {=bool:?}, scr: {=u8:?} }}",
            self.dss(),
            self.frf(),
            self.spo(),
            self.sph(),
            self.scr()
        )
    }
}
#[doc = "Control register 1, SSPCR1 on page 3-5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[must_use]
    #[inline(always)]
    pub const fn lbm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub const fn set_lbm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn sse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled."]
    #[inline(always)]
    pub const fn set_sse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave."]
    #[must_use]
    #[inline(always)]
    pub const fn ms(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave."]
    #[inline(always)]
    pub const fn set_ms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sod(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode."]
    #[inline(always)]
    pub const fn set_sod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
impl core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr1")
            .field("lbm", &self.lbm())
            .field("sse", &self.sse())
            .field("ms", &self.ms())
            .field("sod", &self.sod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr1 {{ lbm: {=bool:?}, sse: {=bool:?}, ms: {=bool:?}, sod: {=bool:?} }}",
            self.lbm(),
            self.sse(),
            self.ms(),
            self.sod()
        )
    }
}
#[doc = "DMA control register, SSPDMACR on page 3-12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacr(pub u32);
impl Dmacr {
    #[doc = "Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdmae(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub const fn set_rxdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn txdmae(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub const fn set_txdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Dmacr {
    #[inline(always)]
    fn default() -> Dmacr {
        Dmacr(0)
    }
}
impl core::fmt::Debug for Dmacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmacr")
            .field("rxdmae", &self.rxdmae())
            .field("txdmae", &self.txdmae())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmacr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmacr {{ rxdmae: {=bool:?}, txdmae: {=bool:?} }}",
            self.rxdmae(),
            self.txdmae()
        )
    }
}
#[doc = "Data register, SSPDR on page 3-6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
impl core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dr {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "Interrupt clear register, SSPICR on page 3-11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Clears the SSPRORINTR interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn roric(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clears the SSPRORINTR interrupt"]
    #[inline(always)]
    pub const fn set_roric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears the SSPRTINTR interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rtic(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    pub const fn set_rtic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr")
            .field("roric", &self.roric())
            .field("rtic", &self.rtic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr {{ roric: {=bool:?}, rtic: {=bool:?} }}",
            self.roric(),
            self.rtic()
        )
    }
}
#[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imsc(pub u32);
impl Imsc {
    #[doc = "Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
    #[must_use]
    #[inline(always)]
    pub const fn rorim(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
    #[inline(always)]
    pub const fn set_rorim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
    #[must_use]
    #[inline(always)]
    pub const fn rtim(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
    #[inline(always)]
    pub const fn set_rtim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
    #[must_use]
    #[inline(always)]
    pub const fn rxim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
    #[inline(always)]
    pub const fn set_rxim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
    #[must_use]
    #[inline(always)]
    pub const fn txim(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    pub const fn set_txim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Imsc {
    #[inline(always)]
    fn default() -> Imsc {
        Imsc(0)
    }
}
impl core::fmt::Debug for Imsc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imsc")
            .field("rorim", &self.rorim())
            .field("rtim", &self.rtim())
            .field("rxim", &self.rxim())
            .field("txim", &self.txim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imsc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Imsc {{ rorim: {=bool:?}, rtim: {=bool:?}, rxim: {=bool:?}, txim: {=bool:?} }}",
            self.rorim(),
            self.rtim(),
            self.rxim(),
            self.txim()
        )
    }
}
#[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mis(pub u32);
impl Mis {
    #[doc = "Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rormis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
    #[inline(always)]
    pub const fn set_rormis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rtmis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
    #[inline(always)]
    pub const fn set_rtmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rxmis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
    #[inline(always)]
    pub const fn set_rxmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn txmis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
    #[inline(always)]
    pub const fn set_txmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Mis {
    #[inline(always)]
    fn default() -> Mis {
        Mis(0)
    }
}
impl core::fmt::Debug for Mis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mis")
            .field("rormis", &self.rormis())
            .field("rtmis", &self.rtmis())
            .field("rxmis", &self.rxmis())
            .field("txmis", &self.txmis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mis {{ rormis: {=bool:?}, rtmis: {=bool:?}, rxmis: {=bool:?}, txmis: {=bool:?} }}",
            self.rormis(),
            self.rtmis(),
            self.rxmis(),
            self.txmis()
        )
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcellid0(pub u32);
impl Pcellid0 {
    #[doc = "These bits read back as 0x0D"]
    #[must_use]
    #[inline(always)]
    pub const fn ssppcellid0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x0D"]
    #[inline(always)]
    pub const fn set_ssppcellid0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Pcellid0 {
    #[inline(always)]
    fn default() -> Pcellid0 {
        Pcellid0(0)
    }
}
impl core::fmt::Debug for Pcellid0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcellid0")
            .field("ssppcellid0", &self.ssppcellid0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcellid0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pcellid0 {{ ssppcellid0: {=u8:?} }}", self.ssppcellid0())
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcellid1(pub u32);
impl Pcellid1 {
    #[doc = "These bits read back as 0xF0"]
    #[must_use]
    #[inline(always)]
    pub const fn ssppcellid1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0xF0"]
    #[inline(always)]
    pub const fn set_ssppcellid1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Pcellid1 {
    #[inline(always)]
    fn default() -> Pcellid1 {
        Pcellid1(0)
    }
}
impl core::fmt::Debug for Pcellid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcellid1")
            .field("ssppcellid1", &self.ssppcellid1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcellid1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pcellid1 {{ ssppcellid1: {=u8:?} }}", self.ssppcellid1())
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcellid2(pub u32);
impl Pcellid2 {
    #[doc = "These bits read back as 0x05"]
    #[must_use]
    #[inline(always)]
    pub const fn ssppcellid2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x05"]
    #[inline(always)]
    pub const fn set_ssppcellid2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Pcellid2 {
    #[inline(always)]
    fn default() -> Pcellid2 {
        Pcellid2(0)
    }
}
impl core::fmt::Debug for Pcellid2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcellid2")
            .field("ssppcellid2", &self.ssppcellid2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcellid2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pcellid2 {{ ssppcellid2: {=u8:?} }}", self.ssppcellid2())
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcellid3(pub u32);
impl Pcellid3 {
    #[doc = "These bits read back as 0xB1"]
    #[must_use]
    #[inline(always)]
    pub const fn ssppcellid3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0xB1"]
    #[inline(always)]
    pub const fn set_ssppcellid3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Pcellid3 {
    #[inline(always)]
    fn default() -> Pcellid3 {
        Pcellid3(0)
    }
}
impl core::fmt::Debug for Pcellid3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcellid3")
            .field("ssppcellid3", &self.ssppcellid3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcellid3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pcellid3 {{ ssppcellid3: {=u8:?} }}", self.ssppcellid3())
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Periphid0(pub u32);
impl Periphid0 {
    #[doc = "These bits read back as 0x22"]
    #[must_use]
    #[inline(always)]
    pub const fn partnumber0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x22"]
    #[inline(always)]
    pub const fn set_partnumber0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Periphid0 {
    #[inline(always)]
    fn default() -> Periphid0 {
        Periphid0(0)
    }
}
impl core::fmt::Debug for Periphid0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Periphid0")
            .field("partnumber0", &self.partnumber0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Periphid0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Periphid0 {{ partnumber0: {=u8:?} }}",
            self.partnumber0()
        )
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Periphid1(pub u32);
impl Periphid1 {
    #[doc = "These bits read back as 0x0"]
    #[must_use]
    #[inline(always)]
    pub const fn partnumber1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x0"]
    #[inline(always)]
    pub const fn set_partnumber1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "These bits read back as 0x1"]
    #[must_use]
    #[inline(always)]
    pub const fn designer0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x1"]
    #[inline(always)]
    pub const fn set_designer0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Periphid1 {
    #[inline(always)]
    fn default() -> Periphid1 {
        Periphid1(0)
    }
}
impl core::fmt::Debug for Periphid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Periphid1")
            .field("partnumber1", &self.partnumber1())
            .field("designer0", &self.designer0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Periphid1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Periphid1 {{ partnumber1: {=u8:?}, designer0: {=u8:?} }}",
            self.partnumber1(),
            self.designer0()
        )
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Periphid2(pub u32);
impl Periphid2 {
    #[doc = "These bits read back as 0x4"]
    #[must_use]
    #[inline(always)]
    pub const fn designer1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x4"]
    #[inline(always)]
    pub const fn set_designer1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "These bits return the peripheral revision"]
    #[must_use]
    #[inline(always)]
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits return the peripheral revision"]
    #[inline(always)]
    pub const fn set_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Periphid2 {
    #[inline(always)]
    fn default() -> Periphid2 {
        Periphid2(0)
    }
}
impl core::fmt::Debug for Periphid2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Periphid2")
            .field("designer1", &self.designer1())
            .field("revision", &self.revision())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Periphid2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Periphid2 {{ designer1: {=u8:?}, revision: {=u8:?} }}",
            self.designer1(),
            self.revision()
        )
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Periphid3(pub u32);
impl Periphid3 {
    #[doc = "These bits read back as 0x00"]
    #[must_use]
    #[inline(always)]
    pub const fn configuration(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x00"]
    #[inline(always)]
    pub const fn set_configuration(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Periphid3 {
    #[inline(always)]
    fn default() -> Periphid3 {
        Periphid3(0)
    }
}
impl core::fmt::Debug for Periphid3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Periphid3")
            .field("configuration", &self.configuration())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Periphid3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Periphid3 {{ configuration: {=u8:?} }}",
            self.configuration()
        )
    }
}
#[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ris(pub u32);
impl Ris {
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rorris(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
    #[inline(always)]
    pub const fn set_rorris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rtris(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
    #[inline(always)]
    pub const fn set_rtris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rxris(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
    #[inline(always)]
    pub const fn set_rxris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn txris(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
    #[inline(always)]
    pub const fn set_txris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ris {
    #[inline(always)]
    fn default() -> Ris {
        Ris(0)
    }
}
impl core::fmt::Debug for Ris {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ris")
            .field("rorris", &self.rorris())
            .field("rtris", &self.rtris())
            .field("rxris", &self.rxris())
            .field("txris", &self.txris())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ris {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ris {{ rorris: {=bool:?}, rtris: {=bool:?}, rxris: {=bool:?}, txris: {=bool:?} }}",
            self.rorris(),
            self.rtris(),
            self.rxris(),
            self.txris()
        )
    }
}
#[doc = "Status register, SSPSR on page 3-7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty."]
    #[must_use]
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty."]
    #[inline(always)]
    pub const fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full."]
    #[must_use]
    #[inline(always)]
    pub const fn tnf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full."]
    #[inline(always)]
    pub const fn set_tnf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rne(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty."]
    #[inline(always)]
    pub const fn set_rne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full."]
    #[must_use]
    #[inline(always)]
    pub const fn rff(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full."]
    #[inline(always)]
    pub const fn set_rff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[must_use]
    #[inline(always)]
    pub const fn bsy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub const fn set_bsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("tfe", &self.tfe())
            .field("tnf", &self.tnf())
            .field("rne", &self.rne())
            .field("rff", &self.rff())
            .field("bsy", &self.bsy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sr {{ tfe: {=bool:?}, tnf: {=bool:?}, rne: {=bool:?}, rff: {=bool:?}, bsy: {=bool:?} }}" , self . tfe () , self . tnf () , self . rne () , self . rff () , self . bsy ())
    }
}
