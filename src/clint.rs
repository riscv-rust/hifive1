//! Clint

use riscv::{csr, interrupt};
use e310x::CLINT;

pub struct Clint<'a>(pub &'a CLINT);

impl<'a> Clone for Clint<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Copy for Clint<'a> {
}

impl<'a> Clint<'a> {
    /// Read mtime register.
    pub fn get_mtime(&self) -> ::aonclk::Ticks<u64> {
        loop {
            let hi = self.0.mtimeh.read().bits();
            let lo = self.0.mtime.read().bits();
            if hi == self.0.mtimeh.read().bits() {
                return ::aonclk::Ticks(((hi as u64) << 32) | lo as u64);
            }
        }
    }

    /// Write mtime register.
    pub fn set_mtime(&self, time: ::aonclk::Ticks<u64>) {
        unsafe {
            self.0.mtimeh.write(|w| w.bits(time.into_hi()));
            self.0.mtime.write(|w| w.bits(time.into()));
        }
    }

    /// Read mtimecmp register.
    pub fn get_mtimecmp(&self) -> ::aonclk::Ticks<u64> {
        let hi = self.0.mtimecmph.read().bits() as u64;
        let lo = self.0.mtimecmp.read().bits() as u64;
        ::aonclk::Ticks(hi << 32 | lo)
    }

    /// Write mtimecmp register.
    pub fn set_mtimecmp(&self, time: ::aonclk::Ticks<u64>) {
        unsafe {
            self.0.mtimecmph.write(|w| w.bits(time.into_hi()));
            self.0.mtimecmp.write(|w| w.bits(time.into()));
        }
    }

    /// Read mcycle register.
    pub fn get_mcycle(&self) -> ::coreclk::Ticks<u64> {
        loop {
            let hi = csr::mcycleh.read().bits();
            let lo = csr::mcycle.read().bits();
            if hi == csr::mcycleh.read().bits() {
                return ::coreclk::Ticks(((hi as u64) << 32) | lo as u64);
            }
        }
    }

    /// Write mcycle register.
    pub fn set_mcycle(&self, cycle: ::coreclk::Ticks<u64>) {
        csr::mcycleh.write(|w| w.bits(cycle.into_hi()));
        csr::mcycle.write(|w| w.bits(cycle.into()));
    }

    /// Read minstret register.
    pub fn get_minstret(&self) -> u64 {
        loop {
            let hi = csr::minstreth.read().bits();
            let lo = csr::minstret.read().bits();
            if hi == csr::minstreth.read().bits() {
                return ((hi as u64) << 32) | lo as u64;
            }
        }
    }

    /// Write minstret register.
    pub fn set_minstret(&self, instret: u64) {
        csr::minstreth.write(|w| w.bits((instret >> 32) as u32));
        csr::minstret.write(|w| w.bits(instret as u32));
    }


    /// Enable Machine-Timer interrupt.
    pub fn enable_mtimer(&self) {
        csr::mie.set(|w| w.mtimer());
    }

    /// Disable Machine-Timer interrupt.
    pub fn disable_mtimer(&self) {
        csr::mie.clear(|w| w.mtimer());
    }

    /// Check if the Machine-Timer is interrupt pending.
    pub fn is_mtimer_pending(&self) -> bool {
        csr::mip.read().mtimer()
    }

    /// Measure the coreclk frequency by counting the number of aonclk ticks.
    pub fn measure_coreclk(&self, min_ticks: ::aonclk::Ticks<u64>) -> u32 {
        interrupt::free(|_| {
            let clint = self.0;

            // Don't start measuring until we see an mtime tick
            while clint.mtime.read().bits() == clint.mtime.read().bits() {}

            let start_cycle = self.get_mcycle();
            let start_time = self.get_mtime();

            // Wait for min_ticks to pass
            while start_time + min_ticks > self.get_mtime() {}

            let end_cycle = self.get_mcycle();
            let end_time = self.get_mtime();

            let delta_cycle: u32 = (end_cycle - start_cycle).into();
            let delta_time: u32 = (end_time - start_time).into();

            (delta_cycle / delta_time) * 32768
                + ((delta_cycle % delta_time) * 32768) / delta_time
        })
    }
}

impl<'a> ::hal::Timer for Clint<'a> {
    type Time = ::aonclk::Ticks<u64>;

    fn get_timeout(&self) -> ::aonclk::Ticks<u64> {
        self.get_mtimecmp()
    }

    fn pause(&self) {
        self.disable_mtimer();
    }

    fn restart(&self) {
        self.set_mtime(::aonclk::Ticks(0));
        self.enable_mtimer();
    }

    fn resume(&self) {
        unimplemented!();
    }

    fn set_timeout<T>(&self, timeout: T)
        where
        T: Into<::aonclk::Ticks<u64>>,
    {
        self.disable_mtimer();
        self.set_mtimecmp(timeout.into());
        self.set_mtime(::aonclk::Ticks(0));
        self.enable_mtimer();
    }

    fn wait(&self) -> ::nb::Result<(), !> {
        if self.is_mtimer_pending() {
            Ok(())
        } else {
            Err(::nb::Error::WouldBlock)
        }
    }
}
