#[doc = "Clock prescale register, SSPCPSR on page 3-8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct cpsr(pub u32);
impl cpsr {
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
impl Default for cpsr {
    #[inline(always)]
    fn default() -> cpsr {
        cpsr(0)
    }
}
impl core::fmt::Debug for cpsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cpsr")
            .field("cpsdvsr", &self.cpsdvsr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for cpsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "cpsr {{ cpsdvsr: {=u8:?} }}", self.cpsdvsr())
    }
}
#[doc = "Control register 0, SSPCR0 on page 3-4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct cr0(pub u32);
impl cr0 {
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
impl Default for cr0 {
    #[inline(always)]
    fn default() -> cr0 {
        cr0(0)
    }
}
impl core::fmt::Debug for cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cr0")
            .field("dss", &self.dss())
            .field("frf", &self.frf())
            .field("spo", &self.spo())
            .field("sph", &self.sph())
            .field("scr", &self.scr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "cr0 {{ dss: {=u8:?}, frf: {=u8:?}, spo: {=bool:?}, sph: {=bool:?}, scr: {=u8:?} }}",
            self.dss(),
            self.frf(),
            self.spo(),
            self.sph(),
            self.scr()
        )
    }
}
#[doc = "Control register 1, SSPCR1 on page 3-5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct cr1(pub u32);
impl cr1 {
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
impl Default for cr1 {
    #[inline(always)]
    fn default() -> cr1 {
        cr1(0)
    }
}
impl core::fmt::Debug for cr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cr1")
            .field("lbm", &self.lbm())
            .field("sse", &self.sse())
            .field("ms", &self.ms())
            .field("sod", &self.sod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for cr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "cr1 {{ lbm: {=bool:?}, sse: {=bool:?}, ms: {=bool:?}, sod: {=bool:?} }}",
            self.lbm(),
            self.sse(),
            self.ms(),
            self.sod()
        )
    }
}
#[doc = "DMA control register, SSPDMACR on page 3-12."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct dmacr(pub u32);
impl dmacr {
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
impl Default for dmacr {
    #[inline(always)]
    fn default() -> dmacr {
        dmacr(0)
    }
}
impl core::fmt::Debug for dmacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("dmacr")
            .field("rxdmae", &self.rxdmae())
            .field("txdmae", &self.txdmae())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for dmacr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "dmacr {{ rxdmae: {=bool:?}, txdmae: {=bool:?} }}",
            self.rxdmae(),
            self.txdmae()
        )
    }
}
#[doc = "Data register, SSPDR on page 3-6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct dr(pub u32);
impl dr {
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
impl Default for dr {
    #[inline(always)]
    fn default() -> dr {
        dr(0)
    }
}
impl core::fmt::Debug for dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("dr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "dr {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "Interrupt clear register, SSPICR on page 3-11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct icr(pub u32);
impl icr {
    #[doc = "Clears the SSPRORINTR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn roric(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clears the SSPRORINTR interrupt."]
    #[inline(always)]
    pub const fn set_roric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears the SSPRTINTR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rtic(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clears the SSPRTINTR interrupt."]
    #[inline(always)]
    pub const fn set_rtic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for icr {
    #[inline(always)]
    fn default() -> icr {
        icr(0)
    }
}
impl core::fmt::Debug for icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("icr")
            .field("roric", &self.roric())
            .field("rtic", &self.rtic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for icr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "icr {{ roric: {=bool:?}, rtic: {=bool:?} }}",
            self.roric(),
            self.rtic()
        )
    }
}
#[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct imsc(pub u32);
impl imsc {
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
impl Default for imsc {
    #[inline(always)]
    fn default() -> imsc {
        imsc(0)
    }
}
impl core::fmt::Debug for imsc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("imsc")
            .field("rorim", &self.rorim())
            .field("rtim", &self.rtim())
            .field("rxim", &self.rxim())
            .field("txim", &self.txim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for imsc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "imsc {{ rorim: {=bool:?}, rtim: {=bool:?}, rxim: {=bool:?}, txim: {=bool:?} }}",
            self.rorim(),
            self.rtim(),
            self.rxim(),
            self.txim()
        )
    }
}
#[doc = "Masked interrupt status register, SSPMIS on page 3-11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct mis(pub u32);
impl mis {
    #[doc = "Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rormis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt."]
    #[inline(always)]
    pub const fn set_rormis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rtmis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt."]
    #[inline(always)]
    pub const fn set_rtmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxmis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt."]
    #[inline(always)]
    pub const fn set_rxmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn txmis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt."]
    #[inline(always)]
    pub const fn set_txmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for mis {
    #[inline(always)]
    fn default() -> mis {
        mis(0)
    }
}
impl core::fmt::Debug for mis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("mis")
            .field("rormis", &self.rormis())
            .field("rtmis", &self.rtmis())
            .field("rxmis", &self.rxmis())
            .field("txmis", &self.txmis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for mis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "mis {{ rormis: {=bool:?}, rtmis: {=bool:?}, rxmis: {=bool:?}, txmis: {=bool:?} }}",
            self.rormis(),
            self.rtmis(),
            self.rxmis(),
            self.txmis()
        )
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct pcellid0(pub u32);
impl pcellid0 {
    #[doc = "These bits read back as 0x0D."]
    #[must_use]
    #[inline(always)]
    pub const fn ssppcellid0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x0D."]
    #[inline(always)]
    pub const fn set_ssppcellid0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for pcellid0 {
    #[inline(always)]
    fn default() -> pcellid0 {
        pcellid0(0)
    }
}
impl core::fmt::Debug for pcellid0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pcellid0")
            .field("ssppcellid0", &self.ssppcellid0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for pcellid0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "pcellid0 {{ ssppcellid0: {=u8:?} }}", self.ssppcellid0())
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct pcellid1(pub u32);
impl pcellid1 {
    #[doc = "These bits read back as 0xF0."]
    #[must_use]
    #[inline(always)]
    pub const fn ssppcellid1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0xF0."]
    #[inline(always)]
    pub const fn set_ssppcellid1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for pcellid1 {
    #[inline(always)]
    fn default() -> pcellid1 {
        pcellid1(0)
    }
}
impl core::fmt::Debug for pcellid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pcellid1")
            .field("ssppcellid1", &self.ssppcellid1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for pcellid1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "pcellid1 {{ ssppcellid1: {=u8:?} }}", self.ssppcellid1())
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct pcellid2(pub u32);
impl pcellid2 {
    #[doc = "These bits read back as 0x05."]
    #[must_use]
    #[inline(always)]
    pub const fn ssppcellid2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x05."]
    #[inline(always)]
    pub const fn set_ssppcellid2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for pcellid2 {
    #[inline(always)]
    fn default() -> pcellid2 {
        pcellid2(0)
    }
}
impl core::fmt::Debug for pcellid2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pcellid2")
            .field("ssppcellid2", &self.ssppcellid2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for pcellid2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "pcellid2 {{ ssppcellid2: {=u8:?} }}", self.ssppcellid2())
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct pcellid3(pub u32);
impl pcellid3 {
    #[doc = "These bits read back as 0xB1."]
    #[must_use]
    #[inline(always)]
    pub const fn ssppcellid3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0xB1."]
    #[inline(always)]
    pub const fn set_ssppcellid3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for pcellid3 {
    #[inline(always)]
    fn default() -> pcellid3 {
        pcellid3(0)
    }
}
impl core::fmt::Debug for pcellid3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("pcellid3")
            .field("ssppcellid3", &self.ssppcellid3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for pcellid3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "pcellid3 {{ ssppcellid3: {=u8:?} }}", self.ssppcellid3())
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct periphid0(pub u32);
impl periphid0 {
    #[doc = "These bits read back as 0x22."]
    #[must_use]
    #[inline(always)]
    pub const fn partnumber0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x22."]
    #[inline(always)]
    pub const fn set_partnumber0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for periphid0 {
    #[inline(always)]
    fn default() -> periphid0 {
        periphid0(0)
    }
}
impl core::fmt::Debug for periphid0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("periphid0")
            .field("partnumber0", &self.partnumber0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for periphid0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "periphid0 {{ partnumber0: {=u8:?} }}",
            self.partnumber0()
        )
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct periphid1(pub u32);
impl periphid1 {
    #[doc = "These bits read back as 0x0."]
    #[must_use]
    #[inline(always)]
    pub const fn partnumber1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x0."]
    #[inline(always)]
    pub const fn set_partnumber1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "These bits read back as 0x1."]
    #[must_use]
    #[inline(always)]
    pub const fn designer0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x1."]
    #[inline(always)]
    pub const fn set_designer0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for periphid1 {
    #[inline(always)]
    fn default() -> periphid1 {
        periphid1(0)
    }
}
impl core::fmt::Debug for periphid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("periphid1")
            .field("partnumber1", &self.partnumber1())
            .field("designer0", &self.designer0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for periphid1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "periphid1 {{ partnumber1: {=u8:?}, designer0: {=u8:?} }}",
            self.partnumber1(),
            self.designer0()
        )
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct periphid2(pub u32);
impl periphid2 {
    #[doc = "These bits read back as 0x4."]
    #[must_use]
    #[inline(always)]
    pub const fn designer1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x4."]
    #[inline(always)]
    pub const fn set_designer1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "These bits return the peripheral revision."]
    #[must_use]
    #[inline(always)]
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits return the peripheral revision."]
    #[inline(always)]
    pub const fn set_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for periphid2 {
    #[inline(always)]
    fn default() -> periphid2 {
        periphid2(0)
    }
}
impl core::fmt::Debug for periphid2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("periphid2")
            .field("designer1", &self.designer1())
            .field("revision", &self.revision())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for periphid2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "periphid2 {{ designer1: {=u8:?}, revision: {=u8:?} }}",
            self.designer1(),
            self.revision()
        )
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct periphid3(pub u32);
impl periphid3 {
    #[doc = "These bits read back as 0x00."]
    #[must_use]
    #[inline(always)]
    pub const fn configuration(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x00."]
    #[inline(always)]
    pub const fn set_configuration(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for periphid3 {
    #[inline(always)]
    fn default() -> periphid3 {
        periphid3(0)
    }
}
impl core::fmt::Debug for periphid3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("periphid3")
            .field("configuration", &self.configuration())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for periphid3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "periphid3 {{ configuration: {=u8:?} }}",
            self.configuration()
        )
    }
}
#[doc = "Raw interrupt status register, SSPRIS on page 3-10."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ris(pub u32);
impl ris {
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rorris(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt."]
    #[inline(always)]
    pub const fn set_rorris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rtris(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt."]
    #[inline(always)]
    pub const fn set_rtris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxris(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt."]
    #[inline(always)]
    pub const fn set_rxris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn txris(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt."]
    #[inline(always)]
    pub const fn set_txris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for ris {
    #[inline(always)]
    fn default() -> ris {
        ris(0)
    }
}
impl core::fmt::Debug for ris {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ris")
            .field("rorris", &self.rorris())
            .field("rtris", &self.rtris())
            .field("rxris", &self.rxris())
            .field("txris", &self.txris())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ris {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ris {{ rorris: {=bool:?}, rtris: {=bool:?}, rxris: {=bool:?}, txris: {=bool:?} }}",
            self.rorris(),
            self.rtris(),
            self.rxris(),
            self.txris()
        )
    }
}
#[doc = "Status register, SSPSR on page 3-7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct sr(pub u32);
impl sr {
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
impl Default for sr {
    #[inline(always)]
    fn default() -> sr {
        sr(0)
    }
}
impl core::fmt::Debug for sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("sr")
            .field("tfe", &self.tfe())
            .field("tnf", &self.tnf())
            .field("rne", &self.rne())
            .field("rff", &self.rff())
            .field("bsy", &self.bsy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for sr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "sr {{ tfe: {=bool:?}, tnf: {=bool:?}, rne: {=bool:?}, rff: {=bool:?}, bsy: {=bool:?} }}" , self . tfe () , self . tnf () , self . rne () , self . rff () , self . bsy ())
    }
}
