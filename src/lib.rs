#![no_std]
#![feature(asm)]
#![feature(get_type_id)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(never_type)]
#![feature(used)]

extern crate embedded_hal as hal;
#[macro_use]
extern crate nb;

extern crate riscv;
pub extern crate e310x;

pub mod clint;
pub mod clock;
pub mod gpio;
pub mod led;
pub mod plic;
pub mod pwm;
pub mod rtc;
pub mod serial;
pub mod time;

use core::fmt::Write;
use riscv::interrupt::Nr;

pub use hal::prelude;
pub use riscv::{csr, interrupt};
pub use e310x::Peripherals;
pub use gpio::{PinConfig, PinInterrupt};
pub use clint::Clint;
pub use led::{Red, Green, Blue};
pub use plic::{Priority, Interrupt, Plic};
pub use pwm::{Align, Channel, Pwm};
pub use rtc::{Rtc, RtcConf};
pub use serial::{Serial, Port};
pub use time::UExt;

/// Initializes the clocks, plic and uart0. Returns Peripherals
/// for application specific initialization.
pub fn init<'a>(baud_rate: u32) -> e310x::Peripherals<'a> {
    let peripherals = unsafe { e310x::Peripherals::all() };

    // Setup clocks
    let clint = Clint(peripherals.CLINT);
    let aon_clock = clock::AonClock(peripherals.AONCLK);
    unsafe { aon_clock.use_external(); }
    let clock = clock::CoreClock(peripherals.PRCI);
    unsafe { clock.use_external(&clint); }

    // Initialize UART0
    let serial = Serial(peripherals.UART0);
    serial.init(baud_rate.hz().invert(), peripherals.GPIO0);

    peripherals
}

/// Default trap handler
///
/// Prints trap cause and calls mtimer_trap_handler or plic_trap_handler if
/// necessary.
#[used]
#[no_mangle]
pub fn trap_handler(trap: riscv::csr::Trap) {
    use riscv::csr::{Trap, Interrupt};

    let peripherals = unsafe { e310x::Peripherals::all() };
    let serial = Serial(peripherals.UART0);

    match trap {
        Trap::Interrupt(x) => {
            match x {
                Interrupt::MachineTimer => {
                    writeln!(Port(&serial), "MachineTimer").unwrap();
                    mtimer_trap_handler(&peripherals);
                },
                Interrupt::MachineExternal => {
                    let plic = Plic(peripherals.PLIC);
                    let intr = plic.claim();

                    writeln!(Port(&serial), "ExternalInterrupt {}", intr.nr()).unwrap();
                    plic_trap_handler(&peripherals, &intr);

                    plic.complete(intr);
                }
                x => {
                    writeln!(Port(&serial), "Interrupt {}", x as u32).unwrap();
                },
            }
        },
        Trap::Exception(x) => {
            let mepc = csr::mepc.read().bits();
            writeln!(Port(&serial), "Exception {} at 0x{:x}", x as u32, mepc).unwrap();
        },
    }
}

/// Default MachineTimer Trap Handler
#[no_mangle]
#[linkage = "weak"]
pub fn mtimer_trap_handler(_: &e310x::Peripherals) {}

/// Default MachineExternal Trap Handler
#[no_mangle]
#[linkage = "weak"]
pub fn plic_trap_handler(_: &e310x::Peripherals, _: &Interrupt) {}


macro_rules! ticks_impl {
    ($n:ident, $t:ty, $f:expr) => {
        pub const $n: $t = $f as $t;

        impl Ticks<$t> {
            /// Applies the function `f` to the inner value
            pub fn map<F>(self, f: F) -> Ticks<$t>
                where F: FnOnce($t) -> $t,
            {
                Ticks(f(self.0))
            }
        }

        impl From<Ticks<$t>> for Microseconds<$t> {
            fn from(ticks: Ticks<$t>) -> Microseconds<$t> {
                let divisor: $t = $n / 1_000_000;
                Microseconds(ticks.0 / divisor)
            }
        }

        impl From<Ticks<$t>> for Milliseconds<$t> {
            fn from(ticks: Ticks<$t>) -> Milliseconds<$t> {
                Milliseconds(ticks.0 / ($n / 1_000))
            }
        }

        impl From<Ticks<$t>> for Seconds<$t> {
            fn from(ticks: Ticks<$t>) -> Seconds<$t> {
                Seconds(ticks.0 / $n)
            }
        }

        impl From<IHertz<$t>> for Ticks<$t> {
            fn from(ihz: IHertz<$t>) -> Ticks<$t> {
                Ticks($n / ihz.0)
            }
        }

        impl From<Microseconds<$t>> for Ticks<$t> {
            fn from(us: Microseconds<$t>) -> Ticks<$t> {
                Ticks(us.0 * ($n / 1_000_000))
            }
        }

        impl From<Milliseconds<$t>> for Ticks<$t> {
            fn from(ms: Milliseconds<$t>) -> Ticks<$t> {
                Ticks(ms.0 * ($n / 1_000))
            }
        }

        impl From<Seconds<$t>> for Ticks<$t> {
            fn from(s: Seconds<$t>) -> Ticks<$t> {
                Ticks(s.0 * $n)
            }
        }

        impl Into<$t> for Ticks<$t> {
            fn into(self) -> $t {
                self.0
            }
        }

        impl ::core::ops::Add for Ticks<$t> {
            type Output = Ticks<$t>;

            fn add(self, other: Ticks<$t>) -> Ticks<$t> {
                Ticks(self.0 + other.0)
            }
        }

        impl ::core::ops::Sub for Ticks<$t> {
            type Output = Ticks<$t>;

            fn sub(self, other: Ticks<$t>) -> Ticks<$t> {
                Ticks(self.0 - other.0)
            }
        }
    }
}

macro_rules! frequency {
    ($FREQUENCY:expr) => {
        use time::*;

        /// Unit of time
        #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct Ticks<T>(pub T);

        ticks_impl!(FREQUENCY_32, u32, $FREQUENCY);
        ticks_impl!(FREQUENCY_64, u64, $FREQUENCY);

        impl Into<u32> for Ticks<u64> {
            fn into(self) -> u32 {
                self.0 as u32
            }
        }

        impl Ticks<u64> {
            pub fn into_hi(self) -> u32 {
                (self.0 >> 32) as u32
            }
        }
    }
}

/// Always-On Clock
pub mod aonclk {
    frequency!(32_768);
}

/// Core Clock
pub mod coreclk {
    frequency!(16_000_000);
}
