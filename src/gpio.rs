//! General Purpose I/O

use core::ops::Deref;
use e310x::gpio0;

/// Enumeration of possible pin configurations.
pub enum PinConfig {
    Input,
    InputPullup,
    Output,
    OutputDrive,
    IoFn0,
    IoFn1,
}

/// Enumeration of pin interrupts.
pub enum PinInterrupt {
    Rise,
    Fall,
    High,
    Low,
}

macro_rules! pin {
    ($Pin:ident, $pinx:ident) => (
        pub struct $Pin;

        impl $Pin {
            pub fn init<T>(gpio: &T, config: PinConfig)
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                match config {
                    PinConfig::Input => {
                        gpio.iof_en.modify(|_, w| w.$pinx().bit(false));
                        gpio.pullup.modify(|_, w| w.$pinx().bit(false));
                        gpio.input_en.modify(|_, w| w.$pinx().bit(true));
                    },
                    PinConfig::InputPullup => {
                        gpio.iof_en.modify(|_, w| w.$pinx().bit(false));
                        gpio.pullup.modify(|_, w| w.$pinx().bit(true));
                        gpio.input_en.modify(|_, w| w.$pinx().bit(true));
                    },
                    PinConfig::Output => {
                        gpio.iof_en.modify(|_, w| w.$pinx().bit(false));
                        gpio.drive.modify(|_, w| w.$pinx().bit(false));
                        gpio.output_en.modify(|_, w| w.$pinx().bit(true));
                    },
                    PinConfig::OutputDrive => {
                        gpio.iof_en.modify(|_, w| w.$pinx().bit(false));
                        gpio.drive.modify(|_, w| w.$pinx().bit(true));
                        gpio.output_en.modify(|_, w| w.$pinx().bit(true));
                    },
                    PinConfig::IoFn0 => {
                        gpio.iof_sel.modify(|_, w| w.$pinx().bit(false));
                        gpio.iof_en.modify(|_, w| w.$pinx().bit(true));
                    },
                    PinConfig::IoFn1 => {
                        gpio.iof_sel.modify(|_, w| w.$pinx().bit(true));
                        gpio.iof_en.modify(|_, w| w.$pinx().bit(true));
                    },
                }
            }

            pub fn read<T>(gpio: &T) -> bool
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                gpio.value.read().$pinx().bit()
            }

            pub fn write<T>(gpio: &T, value: bool)
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                match value {
                    true => $Pin::high(gpio),
                    false => $Pin::low(gpio),
                }
            }

            pub fn high<T>(gpio: &T)
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                gpio.port.modify(|_, w| w.$pinx().bit(true));
            }

            pub fn low<T>(gpio: &T)
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                gpio.port.modify(|_, w| w.$pinx().bit(false));
            }

            pub fn toggle<T>(gpio: &T)
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                gpio.port.modify(|r, w| w.$pinx().bit(!r.$pinx().bit()));
            }

            pub fn enable_interrupt<T>(gpio: &T, intr: PinInterrupt)
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                match intr {
                    PinInterrupt::Rise =>
                        gpio.rise_ie.modify(|_, w| w.$pinx().bit(true)),
                    PinInterrupt::Fall =>
                        gpio.fall_ie.modify(|_, w| w.$pinx().bit(true)),
                    PinInterrupt::High =>
                        gpio.high_ie.modify(|_, w| w.$pinx().bit(true)),
                        PinInterrupt::Low =>
                        gpio.low_ie.modify(|_, w| w.$pinx().bit(true)),
                };
            }

            pub fn disable_interrupt<T>(gpio: &T, intr: PinInterrupt)
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                match intr {
                    PinInterrupt::Rise =>
                        gpio.rise_ie.modify(|_, w| w.$pinx().bit(false)),
                    PinInterrupt::Fall =>
                        gpio.fall_ie.modify(|_, w| w.$pinx().bit(false)),
                    PinInterrupt::High =>
                        gpio.high_ie.modify(|_, w| w.$pinx().bit(false)),
                    PinInterrupt::Low =>
                        gpio.low_ie.modify(|_, w| w.$pinx().bit(false)),
                };
            }

            pub fn clear_pending<T>(gpio: &T, intr: PinInterrupt)
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                match intr {
                    PinInterrupt::Rise =>
                        gpio.rise_ip.write(|w| w.$pinx().bit(true)),
                    PinInterrupt::Fall =>
                        gpio.fall_ip.write(|w| w.$pinx().bit(true)),
                    PinInterrupt::High =>
                        gpio.high_ip.write(|w| w.$pinx().bit(true)),
                    PinInterrupt::Low =>
                        gpio.low_ip.write(|w| w.$pinx().bit(true)),
                }
            }

            pub fn is_interrupt_pending<T>(gpio: &T, intr: PinInterrupt) -> bool
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                match intr {
                    PinInterrupt::Rise =>
                        gpio.rise_ip.read().$pinx().bit(),
                    PinInterrupt::Fall =>
                        gpio.fall_ip.read().$pinx().bit(),
                    PinInterrupt::High =>
                        gpio.high_ip.read().$pinx().bit(),
                    PinInterrupt::Low =>
                        gpio.low_ip.read().$pinx().bit(),
                }
            }

            pub fn is_inverted<T>(gpio: &T) -> bool
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                gpio.out_xor.read().$pinx().bit()
            }

            pub fn set_invert<T>(gpio: &T, value: bool)
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                gpio.out_xor.modify(|_, w| w.$pinx().bit(value));
            }

            pub fn invert<T>(gpio: &T)
                where
                T: Deref<Target = gpio0::RegisterBlock>,
            {
                gpio.out_xor.modify(|r, w| w.$pinx().bit(!r.$pinx().bit()));
            }
        }
    )
}

pin!(Pin0, pin0);
pin!(Pin1, pin1);
pin!(Pin2, pin2);
pin!(Pin3, pin3);
pin!(Pin4, pin4);
pin!(Pin5, pin5);
pin!(Pin6, pin6);
pin!(Pin7, pin7);
pin!(Pin8, pin8);
pin!(Pin9, pin9);
pin!(Pin10, pin10);
pin!(Pin11, pin11);
pin!(Pin12, pin12);
pin!(Pin13, pin13);
pin!(Pin14, pin14);
pin!(Pin15, pin15);
pin!(Pin16, pin16);
pin!(Pin17, pin17);
pin!(Pin18, pin18);
pin!(Pin19, pin19);
pin!(Pin20, pin20);
pin!(Pin21, pin21);
pin!(Pin22, pin22);
pin!(Pin23, pin23);
pin!(Pin24, pin24);
pin!(Pin25, pin25);
pin!(Pin26, pin26);
pin!(Pin27, pin27);
pin!(Pin28, pin28);
pin!(Pin29, pin29);
pin!(Pin30, pin30);
pin!(Pin31, pin31);
