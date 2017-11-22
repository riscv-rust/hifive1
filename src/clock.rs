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
    /// Use external real time oscillator.
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
    /// Use external oscillator with bypassed pll.
    pub unsafe fn use_external(&self, clint: &Clint) {
        self.init_pll(clint, |_, w| {
            // bypass PLL
            w.bypass().bit(true)
                // select HFXOSC
                .refsel().bit(true)
        }, |w| w.divby1().bit(true));
        // Disable HFROSC to save power
        self.0.hfrosccfg.write(|w| w.enable().bit(false));
    }

    /// Use external oscillator with pll. Sets PLL
    /// r=2, f=64, q=2 values to maximum allowable
    /// for a 16MHz reference clock. Output frequency
    /// is 16MHz / 2 * 64 / 2 = 256MHz.
    /// NOTE: By trimming the internal clock to 12MHz
    /// and using r=1, f=64, q=2 the maximum frequency
    /// of 384MHz can be reached.
    pub unsafe fn use_pll(&self, clint: &Clint) {
        self.init_pll(clint, |_, w| {
            // bypass PLL
            w.bypass().bit(false)
            // select HFXOSC
                .refsel().bit(true)
                // bits = r - 1
                .pllr().bits(1)
                // bits = f / 2 - 1
                .pllf().bits(31)
                // bits = q=2 -> 1, q=4 -> 2, q=8 -> 3
                .pllq().bits(1)
        }, |w| w.divby1().bit(true));
        // Disable HFROSC to save power
        self.0.hfrosccfg.write(|w| w.enable().bit(false));
    }

    /// Compute PLL multiplier.
    pub fn pll_mult(&self) -> u32 {
        let pllcfg = self.0.pllcfg.read();
        let plloutdiv = self.0.plloutdiv.read();

        let r = pllcfg.pllr().bits() as u32 + 1;
        let f = (pllcfg.pllf().bits() as u32 + 1) * 2;
        let q = [2, 4, 8][pllcfg.pllq().bits() as usize - 1];

        let div = match plloutdiv.divby1().bit() {
            true => 1,
            false => (plloutdiv.div().bits() as u32 + 1) * 2,
        };

        f / r / q / div
    }

    /// Wait for the pll to lock.
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

    unsafe fn init_pll<F, G>(&self, clint: &Clint, pllcfg: F, plloutdiv: G)
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
    }

    /// Use internal oscillator with bypassed pll.
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

    /// Measure the frequency of coreclk.
    pub fn measure(&self, clint: &Clint) -> u32 {
        // warm up I$
        clint.measure_coreclk(::aonclk::Ticks(1));
        // measure for real
        clint.measure_coreclk(::aonclk::Ticks(10))
    }
}
