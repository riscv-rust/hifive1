#![no_std]

extern crate hifive;

use core::fmt::Write;
use hifive::{Port, Serial};

fn main() {
    let peripherals = hifive::init(115_200);

    let serial = Serial(peripherals.UART0);
    writeln!(Port(&serial), "hello world!").unwrap();
}
