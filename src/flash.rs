//! On-board SPI Flash

use e310x_hal::clock::Clocks;
use e310x_hal::e310x::QSPI0;

/// Configure SPI Flash interface to maximum supported speed
#[inline(always)]
pub fn configure_spi_flash(qspi: &QSPI0, clocks: &Clocks) {
    unsafe {
        extern "C" {
            fn _setup_is25lp(dummy8: bool);
        }

        if clocks.coreclk().0 <= 208_000_000 {
            _setup_is25lp(false)
        } else {
            _setup_is25lp(true)
        }
    }
    qspi.sckdiv.modify(|_, w| unsafe { w.div().bits(0) });
}
