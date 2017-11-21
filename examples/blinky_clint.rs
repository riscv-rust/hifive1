#![no_std]

extern crate hifive;

use hifive::prelude::*;
use hifive::{interrupt, led, Blue, Clint, Peripherals, UExt};

fn main() {
    let peripherals = hifive::init(115_200);
    led::init(peripherals.GPIO0);

    let timer = Clint(peripherals.CLINT);
    timer.set_timeout(1.s());

    unsafe {
        interrupt::enable();
    }
}

#[no_mangle]
pub fn mtimer_trap_handler(p: &Peripherals) {
    Clint(p.CLINT).restart();
    Blue::toggle(p.GPIO0);
}
