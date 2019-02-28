//! On-board user LEDs
//!
//! - Red = Pin 22
//! - Green = Pin 19
//! - Blue = Pin 21
use hal::prelude::*;
use hal::gpio::gpio0::{Pin19, Pin21, Pin22, OUTPUT_EN, DRIVE,
                       OUT_XOR, IOF_EN};
use hal::gpio::{Output, Regular, Invert};

/// Red LED
pub type RED = Pin22<Output<Regular<Invert>>>;

/// Green LED
pub type GREEN = Pin19<Output<Regular<Invert>>>;

/// Blue LED
pub type BLUE = Pin21<Output<Regular<Invert>>>;

/// Returns RED, GREEN and BLUE LEDs.
pub fn rgb<X, Y, Z>(
    red: Pin22<X>, green: Pin19<Y>, blue: Pin21<Z>,
    output_en: &mut OUTPUT_EN, drive: &mut DRIVE,
    out_xor: &mut OUT_XOR, iof_en: &mut IOF_EN
) -> (RED, GREEN, BLUE)
{
    let red: RED = red.into_inverted_output(
        output_en,
        drive,
        out_xor,
        iof_en,
    );
    let green: GREEN = green.into_inverted_output(
        output_en,
        drive,
        out_xor,
        iof_en,
    );
    let blue: BLUE = blue.into_inverted_output(
        output_en,
        drive,
        out_xor,
        iof_en,
    );
    (red, green, blue)
}

/// Generic LED
pub trait Led {
    /// Turns the LED off
    fn off(&mut self);

    /// Turns the LED on
    fn on(&mut self);
}

impl Led for RED {
    fn off(&mut self) {
        _embedded_hal_digital_OutputPin::set_low(self);
    }

    fn on(&mut self) {
        _embedded_hal_digital_OutputPin::set_high(self);
    }
}

impl Led for GREEN {
    fn off(&mut self) {
        _embedded_hal_digital_OutputPin::set_low(self);
    }

    fn on(&mut self) {
        _embedded_hal_digital_OutputPin::set_high(self);
    }
}

impl Led for BLUE {
    fn off(&mut self) {
        _embedded_hal_digital_OutputPin::set_low(self);
    }

    fn on(&mut self) {
        _embedded_hal_digital_OutputPin::set_high(self);
    }
}
