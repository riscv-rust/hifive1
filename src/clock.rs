//! Board-specific clock configuration

use e310x_hal::{
    clock::{AonExt, Clocks, PrciExt},
    e310x::{AONCLK, PRCI},
    time::Hertz,
};

#[cfg(any(
    feature = "board-hifive1",
    feature = "board-hifive1-revb",
    feature = "board-redv"
))]
/// Configures clock generation system.
///
/// For HiFive1 and HiFive1 Rev B boards external oscillators are enabled for
/// both high-frequency and low-frequency clocks.
pub fn configure(prci: PRCI, aonclk: AONCLK, target_coreclk: Hertz) -> Clocks {
    let coreclk = prci.constrain();
    let coreclk = coreclk
        .use_external(Hertz(16_000_000))
        .coreclk(target_coreclk);

    let aonclk = aonclk.constrain();
    let aonclk = aonclk.use_external(Hertz(32_768));

    Clocks::freeze(coreclk, aonclk)
}

#[cfg(any(feature = "board-lofive", feature = "board-lofive-r1"))]
/// Configures clock generation system.
///
/// For the LoFive and LoFive R1 boards, external oscillator is enabled for
/// high-frequency clock. For low-frequency clock internal oscillator is used.
pub fn configure(prci: PRCI, aonclk: AONCLK, target_coreclk: Hertz) -> Clocks {
    let coreclk = prci.constrain();
    let coreclk = coreclk
        .use_external(Hertz(16_000_000))
        .coreclk(target_coreclk);

    let aonclk = aonclk.constrain();

    Clocks::freeze(coreclk, aonclk)
}
