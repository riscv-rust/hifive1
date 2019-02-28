//! Single UART hooked up to FTDI
//!
//! - Tx = Pin 17
//! - Rx = Pin 16
use hal::gpio::gpio0::{Pin16, Pin17, OUT_XOR, IOF_SEL, IOF_EN};
use hal::gpio::{IOF0, NoInvert};
use hal::serial::{Tx, Rx};
use hal::e310x::UART0;

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
    tx: Pin17<X>, rx: Pin16<Y>,
    out_xor: &mut OUT_XOR, iof_sel: &mut IOF_SEL,
    iof_en: &mut IOF_EN
) -> (TxPin, RxPin)
{
    let tx: TxPin = tx.into_iof0(out_xor, iof_sel, iof_en);
    let rx: RxPin = rx.into_iof0(out_xor, iof_sel, iof_en);
    (tx, rx)
}
