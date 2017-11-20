//! Clock configuration
use e310x::{AONCLK, prci, PRCI};
use clint::Clint;

/// Aon Clock interface
pub struct AonClock<'a>(pub &'a AONCLK);

impl<'a> Clone for AonClock<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Copy for AonClock<'a> {}

impl<'a> AonClock<'a> {
    /// Use external real time oscillator
    pub unsafe fn use_external(&self) {
        // The G000 doesn't have a LFXOSC and is hardwired
        // to use the an external oscillator.
        // Disable unused LFROSC to save power.
        self.0.lfrosccfg.write(|w| w.enable().bit(false));
    }
}

/// Core Clock interface
pub struct CoreClock<'a>(pub &'a PRCI);

impl<'a> Clone for CoreClock<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> Copy for CoreClock<'a> {
}

impl<'a> CoreClock<'a> {
    pub unsafe fn use_external(&self, clint: &Clint) {
        self.use_pll(clint, |_, w| {
            // bypass PLL
            w.bypass().bit(true)
                // select HFXOSC
                .refsel().bit(true)
        }, |w| w.divby1().bit(true));
    }

    unsafe fn wait_for_lock(&self, clint: &Clint) {
        // Won't lock when bypassed and will loop forever
        if !self.0.pllcfg.read().bypass().bit_is_set() {
            // Wait for PLL Lock
            // Note that the Lock signal can be glitchy.
            // Need to wait 100 us
            // RTC is running at 32kHz.
            // So wait 4 ticks of RTC.
            let time = clint.get_mtime() + ::aonclk::Ticks(4);
            while clint.get_mtime() < time {}
            // Now it is safe to check for PLL Lock
            while !self.0.pllcfg.read().lock().bit_is_set() {}
        }
    }

    pub unsafe fn use_pll<F, G>(&self, clint: &Clint, pllcfg: F, plloutdiv: G)
        where
        for<'w> F: FnOnce(&prci::pllcfg::R,
                          &'w mut prci::pllcfg::W) -> &'w mut prci::pllcfg::W,
        for<'w> G: FnOnce(&'w mut prci::plloutdiv::W) -> &'w mut prci::plloutdiv::W,
    {
        // Make sure we are running of internal clock
        // before configuring the PLL.
        self.use_internal();
        // Enable HFXOSC
        self.0.hfxosccfg.write(|w| w.enable().bit(true));
        // Wait for HFXOSC to stabilize
        while !self.0.hfxosccfg.read().ready().bit_is_set() {}
        // Configure PLL
        self.0.pllcfg.modify(pllcfg);
        self.0.plloutdiv.write(plloutdiv);
        // Wait for PLL lock
        self.wait_for_lock(clint);
        // Switch to PLL
        self.0.pllcfg.modify(|_, w| {
            w.sel().bit(true)
        });
        // Disable HFROSC to save power
        self.0.hfrosccfg.write(|w| w.enable().bit(false));
    }

    pub unsafe fn use_internal(&self) {
        // Enable HFROSC
        self.0.hfrosccfg.write(|w| {
            w.enable().bit(true)
            // It is OK to change this even if we are running off of it.
            // Reset them to default values.
                .div().bits(4)
                .trim().bits(16)
        });
        // Wait for HFROSC to stabilize
        while !self.0.hfrosccfg.read().ready().bit_is_set() {}
        // Switch to HFROSC
        self.0.pllcfg.modify(|_, w| {
            w.sel().bit(false)
        });
        // Bypass PLL to save power
        self.0.pllcfg.modify(|_, w| {
            w.bypass().bit(true)
            // Select HFROSC as PLL ref to disable HFXOSC later
                .refsel().bit(false)
        });
        // Disable HFXOSC to save power.
        self.0.hfxosccfg.write(|w| w.enable().bit(false));
    }

    pub fn measure(&self, clint: &Clint) -> u32 {
        // warm up I$
        clint.measure_coreclk(::aonclk::Ticks(1));
        // measure for real
        clint.measure_coreclk(::aonclk::Ticks(10))
    }
}
