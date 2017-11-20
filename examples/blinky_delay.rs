#![no_std]

#[macro_use]
extern crate nb;
extern crate hifive;

use hifive::prelude::*;
use hifive::{led, Red, Green, Blue, Clint, UExt};

fn delay(clint: &Clint) {
    block!(clint.wait()).unwrap();
    clint.restart();
}

fn main() {
    let peripherals = hifive::init(115_200);
    led::init(peripherals.GPIO0);

    let clint = Clint(peripherals.CLINT);
    clint.set_timeout(500.ms());

    let gpio = peripherals.GPIO0;
    loop {
        Red::on(gpio);
        delay(&clint);
        Red::off(gpio);
        Green::on(gpio);
        delay(&clint);
        Green::off(gpio);
        Blue::on(gpio);
        delay(&clint);
        Blue::off(gpio);
    }
}
