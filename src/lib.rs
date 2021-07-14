//! Board support crate for HiFive1 and LoFive boards

#![deny(missing_docs)]
#![no_std]

pub use e310x_hal as hal;

pub mod clock;
pub use clock::configure as configure_clocks;

pub mod flash;

#[cfg(any(
    feature = "board-hifive1",
    feature = "board-hifive1-revb",
    feature = "board-redv"
))]
pub mod led;
#[cfg(any(feature = "board-hifive1", feature = "board-hifive1-revb"))]
pub use led::{rgb, Led, BLUE, GREEN, RED};
#[cfg(feature = "board-redv")]
pub use led::{Led, BLUE};

pub mod stdout;
pub use stdout::configure as configure_stdout;

#[doc(hidden)]
pub mod gpio;
