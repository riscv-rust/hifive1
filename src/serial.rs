//! Single UART hooked up to FTDI
//!
//! - Tx = Pin 17
//! - Rx = Pin 16
use e310x_hal::gpio::gpio0::{Pin16, Pin17};
use e310x_hal::gpio::{IOF0, NoInvert};
use e310x_hal::serial::{Tx, Rx};
use e310x_hal::e310x::UART0;

/// UART0 TX Pin
pub type TxPin = Pin17<IOF0<NoInvert>>;
/// UART0 RX Pin
pub type RxPin = Pin16<IOF0<NoInvert>>;
/// UART0 TX
pub type TX = Tx<UART0>;
/// UART0 RX
pub type RX = Rx<UART0>;

/// Return TX, RX pins.
pub fn tx_rx<X, Y>(
    tx: Pin17<X>, rx: Pin16<Y>
) -> (TxPin, RxPin)
{
    let tx: TxPin = tx.into_iof0();
    let rx: RxPin = rx.into_iof0();
    (tx, rx)
}
