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
    writeln!(stdout, "RTC interrupt is set {}", plic.is_enabled(Interrupt::RTC)).unwrap();
    writeln!(stdout, "RTC interrupt nr is {}", Interrupt::RTC.nr()).unwrap();

    unsafe {
        interrupt::enable();
    }

    loop {}
}

#[no_mangle]
pub fn plic_trap_handler(p: &Peripherals, intr: &Interrupt) {
    match *intr {
        Interrupt::RTC => {
            Blue::toggle(p.GPIO0);
        },
        _ => {},
    }
}
