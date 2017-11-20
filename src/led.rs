//! User LEDs
//!
//! - Red = Pin 22
//! - Green = Pin 19
//! - Blue = Pin 21

use e310x::GPIO0;
use gpio::{PinConfig, Pin22, Pin19, Pin21};

pub fn init(gpio: &GPIO0) {
    Pin22::set_invert(gpio, true);
    Pin22::init(gpio, PinConfig::Output);
    Pin19::set_invert(gpio, true);
    Pin19::init(gpio, PinConfig::Output);
    Pin21::set_invert(gpio, true);
    Pin21::init(gpio, PinConfig::Output);
}

#[macro_export]
macro_rules! led {
    ($Color:ident, $Pin:ident) => {
        pub struct $Color;

        impl $Color {
            pub fn on(gpio: &GPIO0) {
                $Pin::high(gpio);
            }

            pub fn off(gpio: &GPIO0) {
                $Pin::low(gpio);
            }

            pub fn toggle(gpio: &GPIO0) {
                $Pin::toggle(gpio);
            }
        }
    }
}

led!(Red, Pin22);
led!(Green, Pin19);
led!(Blue, Pin21);
