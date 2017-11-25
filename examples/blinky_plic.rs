#![no_std]

extern crate hifive;

use core::fmt::Write;
use hifive::*;
use hifive::prelude::*;
use hifive::interrupt::Nr;

fn main() {
    let p = hifive::init(115_200);
    led::init(p.GPIO0);

    Red::on(p.GPIO0);

    let plic = Plic(p.PLIC);
    plic.init();

    RtcConf::new().end(p.RTC);
    Rtc(p.RTC).set_timeout(500.ms());

    plic.set_priority(Interrupt::RTC, Priority::P7);
    plic.enable(Interrupt::RTC);

    let serial = Serial(p.UART0);
    let mut stdout = Port(&serial);
    writeln!(stdout, "External interrupts enabled: {}",
             csr::mie.read().mext()).unwrap();
    let threshold: u32 = plic.get_threshold().into();
    writeln!(stdout, "PLIC threshold priority: {}",
             threshold).unwrap();
    writeln!(stdout, "RTC interrupt number: {}",
             Interrupt::RTC.nr()).unwrap();
    writeln!(stdout, "RTC interrupt enabled: {}",
             plic.is_enabled(Interrupt::RTC)).unwrap();
    let priority: u32 = plic.get_priority(Interrupt::RTC).into();
    writeln!(stdout, "RTC interrupt priority: {}",
             priority).unwrap();

    unsafe {
        interrupt::enable();
    }
}

#[no_mangle]
pub fn plic_trap_handler(p: &Peripherals, intr: &Interrupt) {
    match *intr {
        Interrupt::RTC => {
            Rtc(p.RTC).restart();
            Blue::toggle(p.GPIO0);
        },
        _ => {},
    }
}
