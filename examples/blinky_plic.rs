#![no_std]

extern crate hifive;

use hifive::{interrupt, led, Red, Blue, Green, Interrupt, Plic,
             Channel, Pwm, Peripherals};

fn main() {
    let peripherals = hifive::init(115_200);
    led::init(peripherals.GPIO0);

    let pwm = Pwm(peripherals.PWM0);
    pwm.init();
    pwm.set_cmp(Channel::_0, u16::max_value());
    pwm.set_cmp(Channel::_1, 0);
    pwm.set_cmp(Channel::_2, u16::max_value() / 2);

    let plic = Plic(peripherals.PLIC);
    plic.enable(Interrupt::PWM0CMP0);
    plic.enable(Interrupt::PWM0CMP1);
    plic.enable(Interrupt::PWM0CMP2);
    plic.enable(Interrupt::PWM0CMP3);

    unsafe {
        interrupt::enable();
    }
}

#[no_mangle]
pub fn plic_trap_handler(p: &Peripherals, intr: &Interrupt) {
    //let pwm = Pwm(p.PWM0);

    match *intr {
        Interrupt::PWM0CMP0 => {
            Blue::toggle(p.GPIO0);
        },
        Interrupt::PWM0CMP1 => {
            Green::toggle(p.GPIO0);
        },
        Interrupt::PWM0CMP2 => {
            Red::toggle(p.GPIO0);
        },
        Interrupt::PWM0CMP3 => {

        },
        _ => {},
    }
}
