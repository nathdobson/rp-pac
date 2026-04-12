#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc0Count(pub u32);
impl Proc0Count {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[must_use]
    #[inline(always)]
    pub const fn proc0_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn set_proc0_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Proc0Count {
    #[inline(always)]
    fn default() -> Proc0Count {
        Proc0Count(0)
    }
}
impl core::fmt::Debug for Proc0Count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc0Count")
            .field("proc0_count", &self.proc0_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc0Count {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Proc0Count {{ proc0_count: {=u16:?} }}",
            self.proc0_count()
        )
    }
}
#[doc = "Controls the tick generator."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc0Ctrl(pub u32);
impl Proc0Ctrl {
    #[doc = "start / stop tick generation."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?."]
    #[must_use]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?."]
    #[inline(always)]
    pub const fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Proc0Ctrl {
    #[inline(always)]
    fn default() -> Proc0Ctrl {
        Proc0Ctrl(0)
    }
}
impl core::fmt::Debug for Proc0Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc0Ctrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc0Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Proc0Ctrl {{ enable: {=bool:?}, running: {=bool:?} }}",
            self.enable(),
            self.running()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc0Cycles(pub u32);
impl Proc0Cycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[must_use]
    #[inline(always)]
    pub const fn proc0_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn set_proc0_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Proc0Cycles {
    #[inline(always)]
    fn default() -> Proc0Cycles {
        Proc0Cycles(0)
    }
}
impl core::fmt::Debug for Proc0Cycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc0Cycles")
            .field("proc0_cycles", &self.proc0_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc0Cycles {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Proc0Cycles {{ proc0_cycles: {=u16:?} }}",
            self.proc0_cycles()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc1Count(pub u32);
impl Proc1Count {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[must_use]
    #[inline(always)]
    pub const fn proc1_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn set_proc1_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Proc1Count {
    #[inline(always)]
    fn default() -> Proc1Count {
        Proc1Count(0)
    }
}
impl core::fmt::Debug for Proc1Count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc1Count")
            .field("proc1_count", &self.proc1_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc1Count {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Proc1Count {{ proc1_count: {=u16:?} }}",
            self.proc1_count()
        )
    }
}
#[doc = "Controls the tick generator."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc1Ctrl(pub u32);
impl Proc1Ctrl {
    #[doc = "start / stop tick generation."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?."]
    #[must_use]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?."]
    #[inline(always)]
    pub const fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Proc1Ctrl {
    #[inline(always)]
    fn default() -> Proc1Ctrl {
        Proc1Ctrl(0)
    }
}
impl core::fmt::Debug for Proc1Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc1Ctrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc1Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Proc1Ctrl {{ enable: {=bool:?}, running: {=bool:?} }}",
            self.enable(),
            self.running()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc1Cycles(pub u32);
impl Proc1Cycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[must_use]
    #[inline(always)]
    pub const fn proc1_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn set_proc1_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Proc1Cycles {
    #[inline(always)]
    fn default() -> Proc1Cycles {
        Proc1Cycles(0)
    }
}
impl core::fmt::Debug for Proc1Cycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc1Cycles")
            .field("proc1_cycles", &self.proc1_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc1Cycles {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Proc1Cycles {{ proc1_cycles: {=u16:?} }}",
            self.proc1_cycles()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RiscvCount(pub u32);
impl RiscvCount {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[must_use]
    #[inline(always)]
    pub const fn riscv_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn set_riscv_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for RiscvCount {
    #[inline(always)]
    fn default() -> RiscvCount {
        RiscvCount(0)
    }
}
impl core::fmt::Debug for RiscvCount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RiscvCount")
            .field("riscv_count", &self.riscv_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RiscvCount {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RiscvCount {{ riscv_count: {=u16:?} }}",
            self.riscv_count()
        )
    }
}
#[doc = "Controls the tick generator."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RiscvCtrl(pub u32);
impl RiscvCtrl {
    #[doc = "start / stop tick generation."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?."]
    #[must_use]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?."]
    #[inline(always)]
    pub const fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RiscvCtrl {
    #[inline(always)]
    fn default() -> RiscvCtrl {
        RiscvCtrl(0)
    }
}
impl core::fmt::Debug for RiscvCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RiscvCtrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RiscvCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RiscvCtrl {{ enable: {=bool:?}, running: {=bool:?} }}",
            self.enable(),
            self.running()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RiscvCycles(pub u32);
impl RiscvCycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[must_use]
    #[inline(always)]
    pub const fn riscv_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn set_riscv_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for RiscvCycles {
    #[inline(always)]
    fn default() -> RiscvCycles {
        RiscvCycles(0)
    }
}
impl core::fmt::Debug for RiscvCycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RiscvCycles")
            .field("riscv_cycles", &self.riscv_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RiscvCycles {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RiscvCycles {{ riscv_cycles: {=u16:?} }}",
            self.riscv_cycles()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0Count(pub u32);
impl Timer0Count {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn set_timer0_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Timer0Count {
    #[inline(always)]
    fn default() -> Timer0Count {
        Timer0Count(0)
    }
}
impl core::fmt::Debug for Timer0Count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer0Count")
            .field("timer0_count", &self.timer0_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0Count {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timer0Count {{ timer0_count: {=u16:?} }}",
            self.timer0_count()
        )
    }
}
#[doc = "Controls the tick generator."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0Ctrl(pub u32);
impl Timer0Ctrl {
    #[doc = "start / stop tick generation."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?."]
    #[must_use]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?."]
    #[inline(always)]
    pub const fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Timer0Ctrl {
    #[inline(always)]
    fn default() -> Timer0Ctrl {
        Timer0Ctrl(0)
    }
}
impl core::fmt::Debug for Timer0Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer0Ctrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timer0Ctrl {{ enable: {=bool:?}, running: {=bool:?} }}",
            self.enable(),
            self.running()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0Cycles(pub u32);
impl Timer0Cycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn set_timer0_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Timer0Cycles {
    #[inline(always)]
    fn default() -> Timer0Cycles {
        Timer0Cycles(0)
    }
}
impl core::fmt::Debug for Timer0Cycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer0Cycles")
            .field("timer0_cycles", &self.timer0_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0Cycles {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timer0Cycles {{ timer0_cycles: {=u16:?} }}",
            self.timer0_cycles()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1Count(pub u32);
impl Timer1Count {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn set_timer1_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Timer1Count {
    #[inline(always)]
    fn default() -> Timer1Count {
        Timer1Count(0)
    }
}
impl core::fmt::Debug for Timer1Count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer1Count")
            .field("timer1_count", &self.timer1_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1Count {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timer1Count {{ timer1_count: {=u16:?} }}",
            self.timer1_count()
        )
    }
}
#[doc = "Controls the tick generator."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1Ctrl(pub u32);
impl Timer1Ctrl {
    #[doc = "start / stop tick generation."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?."]
    #[must_use]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?."]
    #[inline(always)]
    pub const fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Timer1Ctrl {
    #[inline(always)]
    fn default() -> Timer1Ctrl {
        Timer1Ctrl(0)
    }
}
impl core::fmt::Debug for Timer1Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer1Ctrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timer1Ctrl {{ enable: {=bool:?}, running: {=bool:?} }}",
            self.enable(),
            self.running()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1Cycles(pub u32);
impl Timer1Cycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn set_timer1_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Timer1Cycles {
    #[inline(always)]
    fn default() -> Timer1Cycles {
        Timer1Cycles(0)
    }
}
impl core::fmt::Debug for Timer1Cycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer1Cycles")
            .field("timer1_cycles", &self.timer1_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1Cycles {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timer1Cycles {{ timer1_cycles: {=u16:?} }}",
            self.timer1_cycles()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WatchdogCount(pub u32);
impl WatchdogCount {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[must_use]
    #[inline(always)]
    pub const fn watchdog_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn set_watchdog_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for WatchdogCount {
    #[inline(always)]
    fn default() -> WatchdogCount {
        WatchdogCount(0)
    }
}
impl core::fmt::Debug for WatchdogCount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WatchdogCount")
            .field("watchdog_count", &self.watchdog_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WatchdogCount {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WatchdogCount {{ watchdog_count: {=u16:?} }}",
            self.watchdog_count()
        )
    }
}
#[doc = "Controls the tick generator."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WatchdogCtrl(pub u32);
impl WatchdogCtrl {
    #[doc = "start / stop tick generation."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?."]
    #[must_use]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?."]
    #[inline(always)]
    pub const fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for WatchdogCtrl {
    #[inline(always)]
    fn default() -> WatchdogCtrl {
        WatchdogCtrl(0)
    }
}
impl core::fmt::Debug for WatchdogCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WatchdogCtrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WatchdogCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WatchdogCtrl {{ enable: {=bool:?}, running: {=bool:?} }}",
            self.enable(),
            self.running()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WatchdogCycles(pub u32);
impl WatchdogCycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[must_use]
    #[inline(always)]
    pub const fn watchdog_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn set_watchdog_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for WatchdogCycles {
    #[inline(always)]
    fn default() -> WatchdogCycles {
        WatchdogCycles(0)
    }
}
impl core::fmt::Debug for WatchdogCycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WatchdogCycles")
            .field("watchdog_cycles", &self.watchdog_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WatchdogCycles {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WatchdogCycles {{ watchdog_cycles: {=u16:?} }}",
            self.watchdog_cycles()
        )
    }
}
