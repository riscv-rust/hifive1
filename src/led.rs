//! On-board user LEDs
//!
//! Hifive1 (+ revB)
//! - Red = Pin 22
//! - Green = Pin 19
//! - Blue = Pin 21
//!
//! RedV
//! - Blue = Pin 5

#[cfg(feature = "board-redv")]
use e310x_hal::gpio::gpio0::Pin5;
#[cfg(any(feature = "board-hifive1", feature = "board-hifive1-revb"))]
use e310x_hal::gpio::gpio0::{Pin19, Pin21, Pin22};
use e310x_hal::gpio::{Invert, Output, Regular};
use embedded_hal::digital::blocking::OutputPin;

#[cfg(any(feature = "board-hifive1", feature = "board-hifive1-revb"))]
/// Red LED
pub type RED = Pin22<Output<Regular<Invert>>>;

#[cfg(any(feature = "board-hifive1", feature = "board-hifive1-revb"))]
/// Green LED
pub type GREEN = Pin19<Output<Regular<Invert>>>;

#[cfg(any(feature = "board-hifive1", feature = "board-hifive1-revb"))]
/// Blue LED
pub type BLUE = Pin21<Output<Regular<Invert>>>;

#[cfg(feature = "board-redv")]
/// Blue LED
pub type BLUE = Pin5<Output<Regular<Invert>>>;

#[cfg(any(feature = "board-hifive1", feature = "board-hifive1-revb"))]
/// Returns RED, GREEN and BLUE LEDs.
pub fn rgb<X, Y, Z>(red: Pin22<X>, green: Pin19<Y>, blue: Pin21<Z>) -> (RED, GREEN, BLUE) {
    let red: RED = red.into_inverted_output();
    let green: GREEN = green.into_inverted_output();
    let blue: BLUE = blue.into_inverted_output();
    (red, green, blue)
}

/// Generic LED
pub trait Led {
    /// Turns the LED off
    fn off(&mut self);

    /// Turns the LED on
    fn on(&mut self);
}

#[cfg(any(feature = "board-hifive1", feature = "board-hifive1-revb"))]
impl Led for RED {
    fn off(&mut self) {
        self.set_low().unwrap();
    }

    fn on(&mut self) {
        self.set_high().unwrap();
    }
}

#[cfg(any(feature = "board-hifive1", feature = "board-hifive1-revb"))]
impl Led for GREEN {
    fn off(&mut self) {
        self.set_low().unwrap();
    }

    fn on(&mut self) {
        self.set_high().unwrap();
    }
}

impl Led for BLUE {
    fn off(&mut self) {
        self.set_low().unwrap();
    }

    fn on(&mut self) {
        self.set_high().unwrap();
    }
}
