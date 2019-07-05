//! Board support crate for HiFive1 and LoFive boards

#![deny(missing_docs)]
#![no_std]

pub use e310x_hal as hal;

pub mod clock;
pub use clock::configure as configure_clocks;

#[cfg(any(feature = "board-hifive1", feature = "board-hifive1-revb"))]
pub mod led;
#[cfg(any(feature = "board-hifive1", feature = "board-hifive1-revb"))]
pub use led::{RED, GREEN, BLUE, rgb, Led};

pub mod board;
pub use board::BoardResources;

pub mod stdout;
pub use stdout::configure as configure_stdout;
