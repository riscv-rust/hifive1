//! Serial interface
//!
//! You can use the `Serial` interface with these UART instances
//!
//! # UART0
//! - TX: Pin 17 IOF0
//! - RX: Pin 16 IOF0
//! - Interrupt::UART0
//!
//! # UART1
//! - TX: Pin 25 IOF0
//! - RX: Pin 24 IOF0
//! - Interrupt::UART1


use core::any::{Any, TypeId};
use core::ops::Deref;
use e310x::{gpio0, GPIO0, uart0, UART0, UART1};
use gpio::{PinConfig, Pin17, Pin16, Pin25, Pin24};

/// IMPLEMENTATION DETAIL
pub unsafe trait Uart: Deref<Target = uart0::RegisterBlock> {
    /// IMPLEMENTATION DETAIL
    type GPIO: Deref<Target = gpio0::RegisterBlock>;
    type Ticks: Into<u32>;
}

unsafe impl Uart for UART0 {
    type GPIO = GPIO0;
    type Ticks = ::coreclk::Ticks<u32>;
}

unsafe impl Uart for UART1 {
    type GPIO = GPIO0;
    type Ticks = ::coreclk::Ticks<u32>;
}

/// Serial interface
pub struct Serial<'a, U>(pub &'a U)
where
    U: Any + Uart;

impl<'a, U> Clone for Serial<'a, U>
where
    U: Any + Uart,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, U> Copy for Serial<'a, U>
where
    U: Any + Uart,
{
}

impl<'a, U> Serial<'a, U>
where
    U: Any + Uart,
{
    /// Initializes the serial interface with a baud rate of `baud_rate` bits
    /// per second
    pub fn init<B>(&self, baud_rate: B, gpio: &U::GPIO)
        where B: Into<U::Ticks>,
    {
        if self.0.get_type_id() == TypeId::of::<UART0>() {
            Pin16::init(gpio, PinConfig::IoFn0);
            Pin17::init(gpio, PinConfig::IoFn0);
        } else if self.0.get_type_id() == TypeId::of::<UART1>() {
            Pin24::init(gpio, PinConfig::IoFn0);
            Pin25::init(gpio, PinConfig::IoFn0);
        }

        unsafe {
            let div = baud_rate.into().into();
            self.0.div.write(|w| w.bits(div));

            self.0.txctrl.write(|w| w.enable().bit(true));
            self.0.rxctrl.write(|w| w.enable().bit(true));
        }
    }
}

impl<'a, U> ::hal::serial::Read<u8> for Serial<'a, U>
where
    U: Any + Uart,
{
    type Error = !;

    fn read(&self) -> ::nb::Result<u8, !> {
        let uart = self.0;
        let rxdata = uart.rxdata.read();

        if rxdata.empty().bit_is_set() {
            Err(::nb::Error::WouldBlock)
        } else {
            Ok(rxdata.data().bits() as u8)
        }
    }
}

impl<'a, U> ::hal::serial::Write<u8> for Serial<'a, U>
where
    U: Any + Uart,
{
    type Error = !;

    fn write(&self, byte: u8) -> ::nb::Result<(), !> {
        let uart = self.0;
        let txdata = uart.txdata.read();

        if txdata.full().bit_is_set() {
            Err(::nb::Error::WouldBlock)
        } else {
            unsafe {
                uart.txdata.write(|w| w.data().bits(byte));
            }
            Ok(())
        }
    }
}

/// Port
pub struct Port<'p, T>(pub &'p T)
    where
    T: 'p;

impl<'p, T> ::core::fmt::Write for Port<'p, T>
    where
    T: ::hal::serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.as_bytes() {
            let res = block!(self.0.write(*byte));

            if res.is_err() {
                return Err(::core::fmt::Error);
            }

            if *byte == '\n' as u8 {
                let res = block!(self.0.write('\r' as u8));

                if res.is_err() {
                    return Err(::core::fmt::Error);
                }
            }
        }
        Ok(())
    }
}
