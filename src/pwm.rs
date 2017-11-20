//! Pulse Width Modulation
//!
//! You can use the `Pwm` interface with these PWM instances
//!
//! # PWM0
//!
//! - CH0: Pin 0 IOF1
//! - CH1: Pin 1 IOF1
//! - CH2: Pin 2 IOF1
//! - CH3: Pin 3 IOF1
//!
//! # PWM1
//!
//! - CH0: Pin 20 IOF1
//! - CH1: Pin 19 IOF1
//! - CH2: Pin 21 IOF1
//! - CH3: Pin 22 IOF1
//!
//! # PWM2
//!
//! - CH0: Pin 10 IOF1
//! - CH1: Pin 11 IOF1
//! - CH2: Pin 12 IOF1
//! - CH3: Pin 13 IOF1

use core::any::{Any, TypeId};
use core::ops::Deref;

use e310x::{pwm0, PWM0, PWM1, PWM2, gpio0, GPIO0};
use gpio::{PinConfig, Pin0, Pin1, Pin2, Pin3, Pin20, Pin19,
           Pin21, Pin22, Pin10, Pin11, Pin12, Pin13};

/// Channel
#[derive(Clone, Copy, Debug)]
pub enum Channel {
    /// CH0
    _0,
    /// CH1
    _1,
    /// CH2
    _2,
    /// CH3
    _3,
}

/// Channel
#[derive(Clone, Copy, Debug)]
pub enum Align {
    /// Left
    Left,
    /// Center
    Center,
    /// Right
    Right,
}

/// IMPLEMENTATION DETAIL
pub unsafe trait PWM: Deref<Target = pwm0::RegisterBlock> {
    /// IMPLEMENTATION DETAIL
    type GPIO: Deref<Target = gpio0::RegisterBlock>;
}

unsafe impl PWM for PWM0 {
    type GPIO = GPIO0;
}

unsafe impl PWM for PWM1 {
    type GPIO = GPIO0;
}

unsafe impl PWM for PWM2 {
    type GPIO = GPIO0;
}

pub struct Pwm<'a, T>(pub &'a T)
    where
    T: 'a;

impl<'a, T> Clone for Pwm<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for Pwm<'a, T> {}

impl<'a, T> Pwm<'a, T>
    where
    T: Any + PWM,
{
    pub fn init(&self) {
        unsafe {
            self.0.cfg.modify(|_, w| {
                w.enalways().bit(true)
                    // set period of 1s
                    .scale().bits(8)
                    .zerocmp().bit(true)
            });
            self.0.count.write(|w| w.bits(0));
        }
    }

    pub fn enable(&self, channel: Channel, align: Align, gpio: &T::GPIO) {
        if self.0.get_type_id() == TypeId::of::<PWM0>() {
            match channel {
                Channel::_0 => {
                    Pin0::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin0::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin0::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin0::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
                Channel::_1 => {
                    Pin1::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin1::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin1::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin1::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
                Channel::_2 => {
                    Pin2::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin2::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin2::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin2::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
                Channel::_3 => {
                    Pin3::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin3::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin3::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin3::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
            }
        } else if self.0.get_type_id() == TypeId::of::<PWM1>() {
            match channel {
                Channel::_0 => {
                    Pin20::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin20::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin20::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin20::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
                Channel::_1 => {
                    Pin19::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin19::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin19::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin19::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
                Channel::_2 => {
                    Pin21::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin21::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin21::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin21::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
                Channel::_3 => {
                    Pin22::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin22::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin22::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin22::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
            }
        } else if self.0.get_type_id() == TypeId::of::<PWM2>() {
            match channel {
                Channel::_0 => {
                    Pin10::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin10::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin10::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin10::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
                Channel::_1 => {
                    Pin11::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin11::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin11::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin11::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
                Channel::_2 => {
                    Pin12::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin12::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin12::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin12::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
                Channel::_3 => {
                    Pin13::init(gpio, PinConfig::IoFn1);
                    match align {
                        Align::Left => {
                            Pin13::set_invert(gpio, true);
                            self.set_center(channel, false);
                        },
                        Align::Center => {
                            Pin13::set_invert(gpio, false);
                            self.set_center(channel, true);
                        },
                        Align::Right => {
                            Pin13::set_invert(gpio, false);
                            self.set_center(channel, false);
                        }
                    }
                },
            }
        }
    }

    pub fn disable(&self, channel: Channel, gpio: &T::GPIO) {
        if self.0.get_type_id() == TypeId::of::<PWM0>() {
            match channel {
                Channel::_0 => Pin0::init(gpio, PinConfig::Input),
                Channel::_1 => Pin1::init(gpio, PinConfig::Input),
                Channel::_2 => Pin2::init(gpio, PinConfig::Input),
                Channel::_3 => Pin3::init(gpio, PinConfig::Input),
            }
        } else if self.0.get_type_id() == TypeId::of::<PWM1>() {
            match channel {
                Channel::_0 => Pin20::init(gpio, PinConfig::Input),
                Channel::_1 => Pin19::init(gpio, PinConfig::Input),
                Channel::_2 => Pin21::init(gpio, PinConfig::Input),
                Channel::_3 => Pin22::init(gpio, PinConfig::Input),
            }
        } else if self.0.get_type_id() == TypeId::of::<PWM2>() {
            match channel {
                Channel::_0 => Pin10::init(gpio, PinConfig::Input),
                Channel::_1 => Pin11::init(gpio, PinConfig::Input),
                Channel::_2 => Pin12::init(gpio, PinConfig::Input),
                Channel::_3 => Pin13::init(gpio, PinConfig::Input),
            }
        }
    }

    pub fn invert(&self, channel: Channel, gpio: &T::GPIO) {
        if self.0.get_type_id() == TypeId::of::<PWM0>() {
            match channel {
                Channel::_0 => Pin0::invert(gpio),
                Channel::_1 => Pin1::invert(gpio),
                Channel::_2 => Pin2::invert(gpio),
                Channel::_3 => Pin3::invert(gpio),
            }
        } else if self.0.get_type_id() == TypeId::of::<PWM1>() {
            match channel {
                Channel::_0 => Pin20::invert(gpio),
                Channel::_1 => Pin19::invert(gpio),
                Channel::_2 => Pin21::invert(gpio),
                Channel::_3 => Pin22::invert(gpio),
            }
        } else if self.0.get_type_id() == TypeId::of::<PWM2>() {
            match channel {
                Channel::_0 => Pin10::invert(gpio),
                Channel::_1 => Pin11::invert(gpio),
                Channel::_2 => Pin12::invert(gpio),
                Channel::_3 => Pin13::invert(gpio),
            }
        }
    }

    pub fn set_period<P>(&self, period: P)
        where P: Into<::coreclk::Ticks<u32>>
    {
        let ticks: u32 = period.into().into();
        let scale = u16::max_value() as u32 / ticks;
        assert!(scale < 0x10);
        ::riscv::asm::ebreak();
        unsafe {
            self.0.cfg.modify(|_, w| w.scale().bits(scale as u8));
        }
    }

    pub fn get_period(&self) -> ::coreclk::Ticks<u32> {
        let scale = self.0.cfg.read().scale().bits();
        ::coreclk::Ticks(scale as u32 * u16::max_value() as u32)
    }

    pub fn align_left(&self, channel: Channel, gpio: &T::GPIO) {
        match channel {
            Channel::_0 => {
                self.0.cfg.modify(|_, w| w.cmp0center().bit(false));
                self.invert(channel, gpio);
            },
            Channel::_1 => {
                self.0.cfg.modify(|_, w| w.cmp1center().bit(false));
                self.invert(channel, gpio);
            },
            Channel::_2 => {
                self.0.cfg.modify(|_, w| w.cmp2center().bit(false));
                self.invert(channel, gpio);
            }
            Channel::_3 => {
                self.0.cfg.modify(|_, w| w.cmp3center().bit(false));
                self.invert(channel, gpio);
            }
        }
    }

    fn set_center(&self, channel: Channel, value: bool) {
        match channel {
            Channel::_0 => self.0.cfg.modify(|_, w| w.cmp0center().bit(value)),
            Channel::_1 => self.0.cfg.modify(|_, w| w.cmp1center().bit(value)),
            Channel::_2 => self.0.cfg.modify(|_, w| w.cmp2center().bit(value)),
            Channel::_3 => self.0.cfg.modify(|_, w| w.cmp3center().bit(value)),
        }
    }

    pub fn get_cmp(&self, channel: Channel) -> u16 {
        match channel {
            Channel::_0 => self.0.cmp0.read().value().bits(),
            Channel::_1 => self.0.cmp1.read().value().bits(),
            Channel::_2 => self.0.cmp2.read().value().bits(),
            Channel::_3 => self.0.cmp3.read().value().bits(),
        }
    }

    pub fn set_cmp(&self, channel: Channel, cmp: u16) {
        unsafe {
            match channel {
                Channel::_0 => self.0.cmp0.write(|w| w.value().bits(cmp)),
                Channel::_1 => self.0.cmp1.write(|w| w.value().bits(cmp)),
                Channel::_2 => self.0.cmp2.write(|w| w.value().bits(cmp)),
                Channel::_3 => self.0.cmp3.write(|w| w.value().bits(cmp)),
            }
        }
    }
}

/*
/// `hal::Pwm` implementation
impl<'a, T> hal::Pwm for Pwm<'a, T>
    where
    T: Any + PWM,
{
    type Channel = Channel;
    type Duty = u16;
    type Time = ::coreclk::Ticks;

    fn get_duty(&self, channel: Channel) -> u16 {
        self.get_cmp(channel)
    }

    fn disable(&self, channel: Channel) {

    }

    fn enable(&self, channel: Channel) {

    }

    fn get_max_duty(&self) -> u16 {
        u16::max_value()
    }

    fn get_period(&self) -> ::coreclk::Ticks {
        self.get_period()
    }

    fn set_duty(&self, channel: Channel, duty: u16) {
        self.set_cmp(channel, duty);
    }

    fn set_period<P>(&self, period: P)
        where
        P: Into<::coreclk::Ticks>,
    {
        self.set_period(period.into());
    }
}

#[allow(unused_variables)]
/// `hal::Timer` implementation
impl<'a, T> hal::Timer for Pwm<'a, T>
    where
    T: Any + PWM
{
    type Time = ::coreclk::Ticks;

    fn get_timeout(&self) -> ::coreclk::Ticks {
        ::coreclk::Ticks(10)
    }

    fn pause(&self) {

    }

    fn restart(&self) {

    }

    fn resume(&self) {

    }

    fn set_timeout<TO>(&self, timeout: TO)
        where
        TO: Into<::coreclk::Ticks>,
    {

    }

    fn wait(&self) -> nb::Result<(), !> {
        Ok(())
    }
}
*/
