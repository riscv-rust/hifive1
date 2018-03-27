//! Board support crate for the Hifive

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub extern crate e310x_hal as hal;

pub use serial::{TX, RX, tx_rx};
pub use led::{RED, GREEN, BLUE, rgb};

pub mod serial {
    //! Single UART hooked up to FTDI
    //!
    //! - Tx = Pin 17
    //! - Rx = Pin 16
    use hal::gpio::gpio0::{Pin16, Pin17, OUT_XOR, IOF_SEL, IOF_EN};
    use hal::gpio::{IOF0, NoInvert};

    /// UART0 TX Pin
    pub type TX = Pin17<IOF0<NoInvert>>;
    /// UART0 RX Pin
    pub type RX = Pin16<IOF0<NoInvert>>;

    /// Return TX, RX pins.
    pub fn tx_rx<X, Y>(
        tx: Pin17<X>, rx: Pin16<Y>,
        out_xor: &mut OUT_XOR, iof_sel: &mut IOF_SEL,
        iof_en: &mut IOF_EN
    ) -> (TX, RX)
    {
        let tx: TX = tx.into_iof0(out_xor, iof_sel, iof_en);
        let rx: RX = rx.into_iof0(out_xor, iof_sel, iof_en);
        (tx, rx)
    }
}

pub mod led {
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
        fn on(&mut self) {
            _embedded_hal_digital_OutputPin::set_high(self);
        }

        fn off(&mut self) {
            _embedded_hal_digital_OutputPin::set_low(self);
        }
    }

    impl Led for GREEN {
        fn on(&mut self) {
            _embedded_hal_digital_OutputPin::set_high(self);
        }

        fn off(&mut self) {
            _embedded_hal_digital_OutputPin::set_low(self);
        }
    }

    impl Led for BLUE {
        fn on(&mut self) {
            _embedded_hal_digital_OutputPin::set_high(self);
        }

        fn off(&mut self) {
            _embedded_hal_digital_OutputPin::set_low(self);
        }
    }
}
