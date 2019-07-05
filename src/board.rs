//! Board resources

use e310x_hal::gpio::{Unknown, gpio0::*};
use e310x_hal::device::{DevicePeripherals, DeviceResources, DeviceGpioPins};
use e310x_hal::core::CorePeripherals;

/// Pins exposed on the Arduino connector
pub struct BoardPins {
    /// DIG0 / UART_RX
    pub dig0: Pin16<Unknown>,

    /// DIG1 / UART_TX
    pub dig1: Pin17<Unknown>,

    /// DIG2
    pub dig2: Pin18<Unknown>,

    /// DIG3 / PWM
    pub dig3: Pin19<Unknown>,

    /// DIG4
    pub dig4: Pin20<Unknown>,

    /// DIG5 / PWM
    pub dig5: Pin21<Unknown>,

    /// DIG6 / PWM
    pub dig6: Pin22<Unknown>,

    /// DIG7
    pub dig7: Pin23<Unknown>,

    /// DIG8
    pub dig8: Pin0<Unknown>,

    /// DIG9 / PWM
    pub dig9: Pin1<Unknown>,

    /// DIG10 / SS / PWM
    pub dig10: Pin2<Unknown>,

    /// DIG11 / MOSI / PWM
    pub dig11: Pin3<Unknown>,

    /// DIG12 / MISO
    pub dig12: Pin4<Unknown>,

    /// DIG13 / SCK
    pub dig13: Pin5<Unknown>,

    /// DIG15
    pub dig15: Pin9<Unknown>,

    /// DIG16
    pub dig16: Pin10<Unknown>,

    /// DIG17
    pub dig17: Pin11<Unknown>,

    /// DIG18 / SDA (RevB)
    pub dig18: Pin12<Unknown>,

    /// DIG19 / SCL (RevB)
    pub dig19: Pin13<Unknown>,
}

impl From<DeviceGpioPins> for BoardPins {
    fn from(pins: DeviceGpioPins) -> Self {
        BoardPins {
            dig0: pins.pin16,
            dig1: pins.pin17,
            dig2: pins.pin18,
            dig3: pins.pin19,
            dig4: pins.pin20,
            dig5: pins.pin21,
            dig6: pins.pin22,
            dig7: pins.pin23,
            dig8: pins.pin0,
            dig9: pins.pin1,
            dig10: pins.pin2,
            dig11: pins.pin3,
            dig12: pins.pin4,
            dig13: pins.pin5,
            dig15: pins.pin9,
            dig16: pins.pin10,
            dig17: pins.pin11,
            dig18: pins.pin12,
            dig19: pins.pin13,
        }
    }
}


/// Board resources
pub struct BoardResources {
    /// Core peripherals
    pub core_peripherals: CorePeripherals,

    /// Device peripherals
    pub peripherals: DevicePeripherals,

    /// Board pins
    pub pins: BoardPins,
}

impl From<DeviceResources> for BoardResources {
    fn from(r: DeviceResources) -> Self {
        BoardResources {
            core_peripherals: r.core_peripherals,
            peripherals: r.peripherals,
            pins: r.pins.into()
        }
    }
}

impl BoardResources {
    /// Returns all the board resources *once*
    #[inline]
    pub fn take() -> Option<Self> {
        DeviceResources::take().map(BoardResources::from)
    }

    /// Unchecked version of `BoardResources::take`
    pub unsafe fn steal() -> Self {
        DeviceResources::steal().into()
    }
}
