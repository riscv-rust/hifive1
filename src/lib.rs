//! Board support crate for the Hifive

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub extern crate e310x_hal as hal;

pub use serial::{TX, RX, TxPin, RxPin, tx_rx};
pub use led::{RED, GREEN, BLUE, rgb, Led};

pub mod serial;
pub mod led;
pub mod clock;
