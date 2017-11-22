#![no_std]

extern crate hifive;

use core::fmt::Write;
use hifive::{clock, Clint, Port, Serial};

fn main() {
    let p = hifive::init(115_200);

    let serial = Serial(p.UART0);
    let mut stdout = Port(&serial);
    writeln!(stdout, "Setting up PLL").unwrap();

    let clint = Clint(p.CLINT);
    let clock = clock::CoreClock(p.PRCI);

    let freq_calc_default = clock.pll_mult() * 16;
    unsafe { clock.use_pll(&clint); }
    let freq_calc = clock.pll_mult() * 16;
    let freq_measured = clock.measure(&clint) / 1_000_000;
    unsafe { clock.use_external(&clint); }

    writeln!(stdout, "Default PLL settings {}MHz", freq_calc_default).unwrap();
    writeln!(stdout, "Measured clock frequency of {}MHz", freq_measured).unwrap();
    writeln!(stdout, "Computed clock frequency of {}MHz", freq_calc).unwrap();
}
