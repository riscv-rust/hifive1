#![no_main]
#![no_std]

/*
 Demonstration on how to use the feature "virq" from e310x-hal.
This feature enables a kind of vectorized interrupt matching for
all 52 the external interrupts that e310x has. It simply offers a convenient
way to handle each interrupt separately with a function called as the interrupt source.
For example, if an interrupt for GPIO0 is received, and a no mangled function called GPIO0()
exists, that function will automatically handle the exception, and it will be automatically
marked as complete by the PLIC.
This can be applied for all the 52 interrupts declared in e310x/interrupts.rs.
*/

extern crate panic_halt;

use hifive1::{
    hal::core::plic::Priority,
    hal::core::CorePeripherals,
    hal::DeviceResources,
    hal::{
        core::plic::{self, INTERRUPT},
        e310x::{
            interrupt::Interrupt,
            plic::{priority, PRIORITY},
            GPIO0, PLIC,
        },
        gpio::gpio0,
        prelude::*,
    },
    pin, sprintln,
};
use riscv::register::mstatus;
use riscv_rt::entry;

/* Handler for the GPIO0 interrupt */
#[no_mangle]
fn GPIO0() {
    sprintln!("We reached the GPIO0 interrupt!");
}

/* Code adapted from https://github.com/riscv-rust/riscv-rust-quickstart/blob/interrupt-test/examples/interrupt.rs*/
#[entry]
fn main() -> ! {
    /* Get the ownership of the device resources singleton */
    let resources = DeviceResources::take().unwrap();
    let peripherals = resources.peripherals;

    /* Configure system clock */
    let sysclock = hifive1::configure_clocks(peripherals.PRCI, peripherals.AONCLK, 64.mhz().into());
    /* Get the board pins */
    let gpio = resources.pins;

    /* Configure stdout for debugging */
    hifive1::stdout::configure(
        peripherals.UART0,
        pin!(gpio, uart0_tx),
        pin!(gpio, uart0_rx),
        115_200.bps(),
        sysclock,
    );

    /* Set GPIO0 (pin 16) as input */
    let gpio0 = pin!(gpio, dig16);
    _ = gpio0.into_pull_up_input();

    /* Wrapper for easy access */
    let mut plic = resources.core_peripherals.plic;

    /* Unsafe block */
    unsafe {
        /* Get raw PLIC pointer */
        let rplic = &*hifive1::hal::e310x::PLIC::ptr();
        let gpio0_index = 7;
        /* Index 7 is the GPIO0 interrupt source, so we modify it's priority */
        for (i, p) in rplic.priority.iter().enumerate() {
            if i > 0 && i < 52 {
                p.write(|w| w.bits(0xffffffff));
            } else {
                /* Clear all other priorities */
                p.write(|w| w.bits(0));
            }
        }
        let gpio_block = &*hifive1::hal::e310x::GPIO0::ptr();
        /* Enable GPIO fall interrupts */
        gpio_block.fall_ie.write(|w| w.bits(0xffffffff));
        /* Clear pending interrupts */
        gpio_block.fall_ip.write(|w| w.bits(0xffffffff));

        /* Activate global interrupts (mie bit) */
        mstatus::set_mie();
        plic.threshold.set(Priority::P1);
        plic.mext.enable();
    }
    loop {}
}
