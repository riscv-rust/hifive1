//! Board support crate for HiFive1 and LoFive boards

#![deny(missing_docs)]
#![no_std]

pub extern crate e310x_hal as hal;

pub mod clock;
#[cfg(feature = "board-hifive1")]
pub mod led;
pub mod serial;

#[cfg(feature = "board-hifive1")]
pub use led::{RED, GREEN, BLUE, rgb, Led};
pub use serial::{TX, RX, TxPin, RxPin, tx_rx};
